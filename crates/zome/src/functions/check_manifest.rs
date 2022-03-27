use std::collections::HashSet;
use std::iter::FromIterator;
use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;

use crate::functions::*;
use crate::entry_kind::*;
use crate::SignalProtocol;


/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn check_manifest(chunk_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   trace!("check_manifest() START {}", chunk_eh);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Find manifest with that chunk_eh
   let maybe_manifest = find_ParcelManifest(chunk_eh)?;
   if maybe_manifest.is_none() {
      trace!("check_manifest() ABORT - Manifest not found");
      return Ok(None);
   }
   /// Find notice with that manifest
   let manifest_eh = hash_entry(maybe_manifest.unwrap())?;
   let maybe_notice = find_notice(manifest_eh.clone())?;
   if maybe_notice.is_none() {
      trace!("check_manifest() ABORT - Notice not found for manifest {}", manifest_eh);
      return Ok(None);
   }
   let notice = maybe_notice.unwrap();
   let notice_eh = hash_entry(notice)?;
   /// Must not already have a ParcelReceived
   let maybe_receipt = query_ParcelReceived(ParcelReceivedQueryField::Notice(notice_eh.clone()))?;
   if let Some(receipt) = maybe_receipt {
      return Ok(Some(receipt.parcel_eh));
   }
   /// Matching notice found. Check if we have all chunks
   let has_all_chunks = has_all_chunks(manifest_eh.clone())?;
   if !has_all_chunks {
      trace!("check_manifest() ABORT - Missing chunks");
      return Ok(None);
   }
   /// All chunks found. Create ParcelReceived
   let received = ParcelReceived {
      notice_eh,
      parcel_eh: manifest_eh,
   };
   let received_eh = hash_entry(received.clone())?;
   let _hh = create_entry(received.clone())?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::ReceivedParcel(received));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(Some(received_eh))
}


///Find manifest with that chunk_eh
pub fn find_ParcelManifest(chunk_eh: EntryHash) -> ExternResult<Option<ParcelManifest>> {
   /// Get all Create ParcelManifest Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(EntryKind::ParcelManifest.as_type());
   let manifests = query(query_args)?;
   for manifest_el in manifests {
      let manifest: ParcelManifest = get_typed_from_el(manifest_el)?;
      if manifest.chunks.contains(&chunk_eh) {
         return Ok(Some(manifest))
      }
   }
   /// Done
   Ok(None)
}


/// Find manifest with that chunk_eh
pub fn find_notice(parcel_eh: EntryHash) -> ExternResult<Option<DeliveryNotice>> {
   /// Get all Create DeliveryNotice Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(EntryKind::DeliveryNotice.as_type());
   let notices = query(query_args)?;
   for notice_el in notices {
      let notice: DeliveryNotice = get_typed_from_el(notice_el)?;
      let summary_eh = notice.summary.parcel_reference.entry_address();
      if summary_eh == parcel_eh {
         return Ok(Some(notice));
      }
   }
   /// Done
   Ok(None)
}


/// Return all ParcelChunks of a ParcelManifest
pub fn has_all_chunks(manifest_eh: EntryHash) -> ExternResult<bool> {
   /// Get ParcelManifest
   let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
   let len =manifest.chunks.len();
   let chunks_set: HashSet<EntryHash> = HashSet::from_iter(manifest.chunks);
   /// Get all Create ParcelChunk Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(false)
      .entry_hashes(chunks_set);
   let chunk_els = query(query_args)?;
   /// Check if all found
   trace!("has_all_chunks: {} == {} ?", chunk_els.len(), len);
   return Ok(chunk_els.len() == len)
}
