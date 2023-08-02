use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Try to retrieve every chunk
pub fn post_commit_ParcelManifest(entry: Entry, manifest_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_ParcelManifest() {:?}", manifest_eh);
   let parcel_manifest = ParcelManifest::try_from(entry)?;
   /// Find notice
   let notices = query_DeliveryNotice(DeliveryNoticeQueryField::Parcel(manifest_eh.clone()))?;
   if notices.is_empty() {
      warn!("No DeliveryNotice found for post-committed ParcelManifest");
      /// Normal if it is its owners
      return Ok(())
   }
   let notice_eh = hash_entry(notices[0].clone())?;
   /// Try to retrieve parcel if it has been accepted
   let mut pairs = Vec::new();
   for chunk_eh in parcel_manifest.chunks.clone() {
      let input = FetchChunkInput {
         chunk_eh,
         notice_eh: notice_eh.clone(),
      };
      let response = call_self("fetch_chunk", input)?;
      debug!("fetch_chunk() response: {:?}", response);
      let output: Option<(ParcelChunk, Option<Link>)> = decode_response(response)?;
      //assert!(matches!(response, ZomeCallResponse::Ok { .. }));
      if let Some(pair) = output {
         pairs.push(pair);
      }
   }
   /// Commit chunks
   let response = call_self("commit_chunks", pairs)?;
   debug!("commit_chunks() response: {:?}", response);
   assert!(matches!(response, ZomeCallResponse::Ok { .. }));
   /// Done
   Ok(())
}
