use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Try to retrieve every chunk
pub fn post_commit_create_PrivateManifest(_sah: &SignedActionHashed, eh: &EntryHash, entry: Entry) -> ExternResult<()> {
   debug!("post_commit_create_PrivateManifest() {:?}", eh);
   let manifest = ParcelManifest::try_from(entry)?;
   /// Find notice
   let notices = query_DeliveryNotice(DeliveryNoticeQueryField::Parcel(eh.clone()))?;
   if notices.is_empty() {
      warn!("No DeliveryNotice found for post-committed ParcelManifest");
      /// Normal if agent is original creator of ParcelManifest
      return Ok(());
   }
   let notice_eh = hash_entry(notices[0].0.clone())?;
   /// Try to retrieve parcel if it has been accepted
   let mut pairs = Vec::new();
   for chunk_eh in manifest.chunks.clone() {
      let input = FetchChunkInput {
         chunk_eh,
         notice_eh: notice_eh.clone(),
      };
      let response = call_self("pull_chunk", input)?;
      let output: Option<(ParcelChunk, Option<Link>)> = decode_response(response)?;
      //assert!(matches!(response, ZomeCallResponse::Ok { .. }));
      if let Some(pair) = output {
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
