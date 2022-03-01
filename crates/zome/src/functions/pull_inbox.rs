use std::collections::HashMap;
use hdk::prelude::*;

use zome_delivery_types::*;
use crate::DeliveryProtocol;
use crate::functions::unpack_item;
use crate::link_kind::*;
use zome_utils::*;
use crate::entry_kind::{EntryKind};
use crate::receive_dm::*;

/// Zome Function
#[hdk_extern]
pub fn pull_inbox(_:()) -> ExternResult<Vec<HeaderHash>> {
   debug!("pull_inbox() START");
   std::panic::set_hook(Box::new(my_panic_hook));
   /// Get all inbox links
   let me = agent_info()?.agent_latest_pubkey;
   let my_agent_eh = EntryHash::from(me.clone());
   let pending_items = get_links_and_load_type::<PendingItem>(
      my_agent_eh.clone(),
      LinkKind::Inbox.as_tag_opt(),
      //false,
   )?;

   // debug!("pull_inbox() items found: {}", pending_items.len());
   // /// Act as is if we received it from a DM
   // for pending_item in pending_items.clone() {
   //    let dm = DirectMessage {
   //       from: pending_item.author.clone(),
   //       msg: DeliveryProtocol::Item(pending_item.clone()),
   //    } ;
   //    let res = receive_delivery_dm(dm);
   //    if let Err(e) = res {
   //       error!("Failed receiving Item from {}: {}", pending_item.author, e);
   //    }
   // }

   // FIXME: DELETE LINKS !!

   /// Convert Each Item
   let mut entry_map = HashMap::new();
   let mut manifest_map = HashMap::new();
   let mut chunk_map = HashMap::new();
   for pending_item in pending_items {
      match pending_item.kind {
         /// Same behavior as if received via DM
         ItemKind::DeliveryReply => {
            let response = receive_dm_reply(pending_item.author.clone(), pending_item);
            /// DELETE lINK
            if response == DeliveryProtocol::Success {
               //let res = delete_link(link_hh);
            }
         },
         ItemKind::ParcelReceived => { receive_dm_reception(pending_item.author.clone(), pending_item);},
         ItemKind::DeliveryNotice => { receive_dm_notice(pending_item.author.clone(), pending_item);},
         /// Behavior specific to DHT
         ItemKind::Entry => {
            let entry: Entry = unpack_item(pending_item.clone(), pending_item.author.clone())?.unwrap();
            let eh = hash_entry(entry.clone())?;
            /// Check if its a Manifest
            if let Entry::App(entry_bytes) = entry.clone() {
               let maybe_manifest = ParcelManifest::try_from(entry_bytes.into_sb().clone());
               //let maybe_manifest = EntryKind::ParcelManifest.into_zome_entry(byes);
               if let Ok(manifest ) = maybe_manifest {
                  manifest_map.insert(eh, manifest);
               } else {
                  entry_map.insert(eh, entry.clone());
               }
            }
         },
         ItemKind::ParcelChunk => {
            let chunk: ParcelChunk = unpack_item(pending_item.clone(), pending_item.author.clone())?.unwrap();
            let eh = hash_entry(chunk.clone())?;
            chunk_map.insert(eh, chunk);
         },
      }
   }
   /// Bail if no parcel received
   let parcel_count = entry_map.len() + manifest_map.len() + chunk_map.len();
   if parcel_count == 0 {
      debug!("pull_inbox() END - No parcel received");
      return Ok(Vec::new())
   }
   debug!("pull_inbox() parcels found: {} {} {}", entry_map.len(), manifest_map.len(), chunk_map.len());

   /// Some parcel received
   /// Check if we accepted them

   /// Get list of entries waiting to be received
   let mut unreceived_entries = HashMap::new();
   let mut unreceived_chunks = Vec::new();
   let received_chunks: Vec<ParcelChunk> = get_all_typed_local(EntryKind::ParcelChunk.as_type())?;
   let mut received_chunks_ehs: Vec<EntryHash> = Vec::new();
   for chunk in received_chunks {
      let chunk_eh = hash_entry(chunk)?;
      received_chunks_ehs.push(chunk_eh);
   }
   let received_parcels: Vec<ParcelReceived> = get_all_typed_local(EntryKind::ParcelReceived.as_type())?;
   let received_parcel_ehs: Vec<EntryHash> = received_parcels.iter().map(|x| x.notice_eh.clone()).collect();
   let my_replies: Vec<DeliveryReply> = get_all_typed_local(EntryKind::DeliveryReply.as_type())?;
   for reply in my_replies {
      if reply.has_accepted && !received_parcel_ehs.contains(&reply.notice_eh) {
         let notice: DeliveryNotice = get_typed_from_eh(reply.notice_eh)?;
         unreceived_entries.insert(notice.parcel_summary.reference.entry_address(), notice.clone());
         /// Get unreceived chunks
         if let ParcelReference::Manifest(manifest_eh) = notice.parcel_summary.reference {
            let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
            for chunk_eh in manifest.chunks {
               if !received_chunks_ehs.contains(&chunk_eh) {
                  unreceived_chunks.push(chunk_eh);
               }
            }
         }
      }
   }
   debug!("pull_inbox() unreceived entries: {}", unreceived_entries.len());
   /// Commit received parcels
   let mut hhs = Vec::new();
   /// Process entries
   for (eh, entry) in entry_map.iter() {
      if let Some(notice) = unreceived_entries.get(eh) {
         let input = CommitParcelInput {
            entry_def_id: notice.parcel_summary.reference.entry_def_id(),
            entry: entry.to_owned(),
         };
         let hh = commit_parcel(input)?;
         hhs.push(hh);
      }
   }
   /// Process manifests
   for (eh, manifest) in manifest_map.iter() {
      if let Some(_notice) = unreceived_entries.get(eh) {
         let hh = create_entry(manifest)?;
         hhs.push(hh);
         manifest.chunks.iter().for_each(|x|unreceived_chunks.push(x.clone()));
      }
   }
   debug!("pull_inbox() unreceived_chunks entries: {}", unreceived_chunks.len());
   /// Process chunks
   for (eh, entry) in chunk_map.iter() {
      if unreceived_chunks.contains(eh) {
         let hh = create_entry(entry)?;
         hhs.push(hh);
      }
   }
   /// Done
   debug!("pull_inbox() END - Received {} parcels", parcel_count);
   Ok(hhs)
}