use std::collections::HashMap;
use hdk::prelude::*;

use crate::{entries::*, utils::*, send_dm, dm_protocol::*, link_kind::*, ParcelReference, EntryKind};


/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn check_manifest(chunk_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   /// Find manifest with that chunk_eh
   let maybe_manifest = find_ParcelManifest(chunk_eh)?;
   if maybe_manifest.is_none() {
      return Ok(None);
   }
   /// Find notice with that manifest
   let manifest_eh = hash_entry(maybe_manifest.unwrap())?;
   let maybe_notice = find_DeliveryNotice(manifest_eh.clone())?;
   if maybe_notice.is_none() {
      return Ok(None);
   }
   let notice = maybe_notice.unwrap();
   let notice_eh = hash_entry(notice)?;
   /// Must not already have a ParcelReceived
   let maybe_receipt = find_ParcelReceived(notice_eh.clone())?;
   if let Some(receipt) = maybe_receipt {
      return Ok(Some(receipt.parcel_eh));
   }
   /// Matching notice found. Check if we have all chunks
   let has_all_chunks = has_all_chunks(manifest_eh.clone())?;
   if !has_all_chunks {
      return Ok(None);
   }
   /// All chunks found. Create ParcelReceived
   let received = ParcelReceived {
      notice_eh,
      parcel_eh: manifest_eh,
   };
   let received_eh = hash_entry(received.clone())?;
   let _hh = create_entry(received)?;
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

///Find manifest with that chunk_eh
pub fn find_DeliveryNotice(parcel_eh: EntryHash) -> ExternResult<Option<DeliveryNotice>> {
   /// Get all Create DeliveryNotice Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(EntryKind::DeliveryNotice.as_type());
   let notices = query(query_args)?;
   for notice_el in notices {
      let notice: DeliveryNotice = get_typed_from_el(notice_el)?;
      if let ParcelReference::Manifest(eh) = notice.parcel_summary.reference.clone() {
         if eh == parcel_eh {
            return Ok(Some(notice));
         }
      }
   }
   /// Done
   Ok(None)
}

/// Return all ParcelChunks of a ParcelManifest
pub fn has_all_chunks(manifest_eh: EntryHash) -> ExternResult<bool> {
   /// Get ParcelManifest
   let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
   /// Get all Create ParcelChunk Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(false)
      .entry_hashes(manifest.chunks.into());
   let chunk_els = query(query_args)?;
   /// Check if all found
   return Ok(chunk_els.len() == manifest.chunks.len())
}


///Find ParcelReceived for given notice
pub fn find_ParcelReceived(notice_eh: EntryHash) -> ExternResult<Option<ParcelReceived>> {
   /// Get all Create ParcelReceived Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(EntryKind::ParcelReceived.as_type());
   let receipts = query(query_args)?;
   for receipt_el in receipts {
      let receipt: ParcelReceived = get_typed_from_el(receipt_el)?;
         if receipt.notice_eh == notice_eh {
            return Ok(Some(receipt));
         }
   }
   /// Done
   Ok(None)
}