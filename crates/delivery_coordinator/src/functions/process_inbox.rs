use std::collections::HashMap;
use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

/// Get All inbox items waiting for this agent (pending links) and process them.
/// Return ActionHashs of parcels committed during the pull
#[hdk_extern]
pub fn process_inbox(_:()) -> ExternResult<Vec<ActionHash>> {
   debug!("START");
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all inbox items
   let pending_pairs = probe_all_inbox_items(None)?;
   debug!("pending items count: {}", pending_pairs.len());
   /// Convert Each Item
   let mut entry_map = HashMap::new();
   let mut manifest_map = HashMap::new();
   let mut chunk_map = HashMap::new();
   for (pending_item, link) in pending_pairs {
      debug!("inbox item: {:?}", pending_item.kind);
      match pending_item.kind {
         /// Same behavior as if received via DM
         ItemKind::NoticeAck => {
            let res = receive_ack(pending_item.author.clone(), pending_item);
            match res {
               Err(e) => warn!("{}", e),
               Ok(_) => { let _res = delete_link_relaxed(link.create_link_hash); },
            }
         }
         ItemKind::NoticeReply => {
            let res = receive_reply(pending_item.author.clone(), pending_item);
            match res {
               Err(e) => warn!("{}", e),
               Ok(_) => { let _res = delete_link_relaxed(link.create_link_hash); },
            }
         }
         ItemKind::ReceptionProof => {
            let res = receive_reception(pending_item.author.clone(), pending_item);
            match res {
               Err(e) => warn!("{}", e),
               Ok(_) => { let _res = delete_link_relaxed(link.create_link_hash); },
            }
         }
         ItemKind::DeliveryNotice => {
            let res = receive_notice(pending_item.author.clone(), pending_item);
            match res {
               Err(e) => warn!("{}", e),
               Ok(_) => { let _res = delete_link_relaxed(link.create_link_hash); },
            }
         },
         /// Behavior specific to DHT
         ItemKind::AppEntryBytes => {
            let entry: Entry = unpack_entry(pending_item.clone(), pending_item.author.clone())?.unwrap();
            let eh = hash_entry(entry.clone())?;
            /// Check if its a Manifest
            if let Entry::App(entry_bytes) = entry.clone() {
               let maybe_manifest = ParcelManifest::try_from(entry_bytes.into_sb().clone());
               //let maybe_manifest = EntryKind::ParcelManifest.into_zome_entry(byes);
               if let Ok(manifest ) = maybe_manifest {
                  manifest_map.insert(eh, manifest);
               } else {
                  entry_map.insert(eh, (entry.clone(), link));
               }
            }
         }
         ItemKind::ParcelChunk => {
            let chunk: ParcelChunk = unpack_item(pending_item.clone(), pending_item.author.clone())?.unwrap();
            let eh = hash_entry(chunk.clone())?;
            chunk_map.insert(eh, (chunk, link));
         },
      }
   }
   /// Bail if no parcel received
   let parcel_count = entry_map.len() + manifest_map.len() + chunk_map.len();
   if parcel_count == 0 {
      debug!("END - No parcel received");
      return Ok(Vec::new())
   }
   debug!("parcels found: {} {} {}", entry_map.len(), manifest_map.len(), chunk_map.len());

   /// Some parcel received
   /// Check if we accepted them

   /// Get list of entries waiting to be received
   let mut unreceived_entries = HashMap::new();
   let mut unreceived_chunks = Vec::new();
   let tuples = get_all_typed_local::<ParcelChunk>(EntryType::App(DeliveryEntryTypes::PrivateChunk.try_into().unwrap()))?;
   let mut received_chunks_ehs: Vec<EntryHash> = Vec::new();
   for (_, _, chunk) in tuples {
      let chunk_eh = hash_entry(chunk)?;
      received_chunks_ehs.push(chunk_eh);
   }
   let tuples = get_all_typed_local::<ReceptionProof>(EntryType::App(DeliveryEntryTypes::ReceptionProof.try_into().unwrap()))?;
   let received_parcel_ehs: Vec<EntryHash> = tuples.iter().map(|(_, _, x)| x.notice_eh.clone()).collect();
   let replies_tuples = get_all_typed_local::<NoticeReply>(EntryType::App(DeliveryEntryTypes::NoticeReply.try_into().unwrap()))?;
   debug!("my_replies: {}", replies_tuples.len());
   for (_, _, reply) in replies_tuples {
      //debug!("process_inbox() reply: {:?}", reply);
      if reply.has_accepted && !received_parcel_ehs.contains(&reply.notice_eh) {
         let notice: DeliveryNotice = get_typed_from_eh(reply.notice_eh)?;
         unreceived_entries.insert(notice.summary.parcel_reference.parcel_eh.clone(), notice.clone());
         /// Get unreceived chunks
         if let ParcelKind::Manifest(_) = notice.summary.parcel_reference.description.kind_info {
            let maybe_manifest: ExternResult<ParcelManifest> = get_typed_from_eh(notice.summary.parcel_reference.parcel_eh);
            /// Manifest might not have been received yet
            if let Ok(manifest) = maybe_manifest {
               for chunk_eh in manifest.chunks {
                  if !received_chunks_ehs.contains(&chunk_eh) {
                     unreceived_chunks.push(chunk_eh);
                  }
               }
            }
         }
      }
   }
   debug!("unreceived entries: {}", unreceived_entries.len());
   /// Commit received parcels
   let mut ahs = Vec::new();
   /// Process entries
   for (eh, (entry, link)) in entry_map.iter() {
      if let Some(notice) = unreceived_entries.get(eh) {
         println!("commit parcel from link: {:?}", link.create_link_hash.clone());

         /// Make sure CreateLink exists
         let maybe_el = get(link.create_link_hash.clone(), GetOptions::default())?;
         if maybe_el.is_none() {
            warn!("CreateLink not found.");
            // return Err(WasmError::Guest("process_inbox(): CreateLink not found.".to_string()));
            continue;
         }

         let ah = call_commit_parcel(
            entry.to_owned(),
            notice,
            Some(link.create_link_hash.clone()),
         )?;
         ahs.push(ah);
      }
   }
   /// Process manifests
   for (eh, manifest) in manifest_map.iter() {
      if let Some(_notice) = unreceived_entries.get(eh) {
         let ah = create_entry_relaxed(DeliveryEntry::PrivateManifest(manifest.clone()))?;
         ahs.push(ah);
         manifest.chunks.iter().for_each(|x|unreceived_chunks.push(x.clone()));
      }
   }
   debug!("unreceived_chunks entries: {}", unreceived_chunks.len());
   /// Process chunks
   for (eh, (entry, link)) in chunk_map.iter() {
      if unreceived_chunks.contains(eh) {
         let ah = create_entry_relaxed(DeliveryEntry::PrivateChunk(entry.clone()))?;
         let _link_hh = delete_link_relaxed(link.create_link_hash.clone())?;
         ahs.push(ah);
      }
   }
   /// Done
   debug!("END - Received {} parcels ({})", parcel_count, ahs.len());
   Ok(ahs)
}
