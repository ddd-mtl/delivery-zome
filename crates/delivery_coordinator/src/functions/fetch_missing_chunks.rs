use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Try to retrieve every chunk
#[hdk_extern]
pub fn fetch_missing_chunks(manifest_eh: EntryHash) -> ExternResult<()> {
   debug!("START - {:?}", manifest_eh);
   /// Find chunks
   let missing_chunks = check_manifest_integrity(manifest_eh.clone())?;
   if missing_chunks.is_empty() {
      return Ok(());
   }
   debug!("missing_chunks: {}", missing_chunks.len());
   /// Private: Find notice
   let _manifest = get_typed_from_eh::<ParcelManifest>(manifest_eh.clone())?;
   let notices = query_DeliveryNotice(DeliveryNoticeQueryField::Parcel(manifest_eh.clone()))?;
   if notices.is_empty() {
      debug!("No Notice found for post-committed ParcelManifest");
      /// Normal if agent is original creator of ParcelManifest
      return Ok(())
   }
   let notice = notices[0].0.clone();
   let notice_eh = hash_entry(notice.clone())?;
   debug!("notice_eh: {:?}", notice_eh);
   ///
   let mut pairs = Vec::new();
   for chunk_eh in missing_chunks.clone() {
      debug!("fetching chunk {} ...", pairs.len());
      let maybe_chunk = pull_chunk(chunk_eh.clone(), notice.clone())?;
      if let Some(pair) = maybe_chunk {
         pairs.push(pair);
      }
   }
   /// Commit chunks
   let response = call_self("commit_received_chunks", pairs)?;
   debug!("commit_received_chunks() response: {:?}", response);
   assert!(matches!(response, ZomeCallResponse::Ok { .. }));
   /// Done
   Ok(())
}