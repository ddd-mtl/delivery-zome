use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Check if each private manifest is complete
/// Return EntryHash of every incomplete manifests
#[hdk_extern]
pub fn scan_incomplete_manifests(_: ()) -> ExternResult<Vec<EntryHash>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let tuples = query_all_private_manifests(())?;
   let mut incomplete_manifests = Vec::new();
   for tuple in tuples {
      let chunks = check_manifest_integrity(tuple.0.clone())?;
      if !chunks.is_empty() {
         incomplete_manifests.push(tuple.0);
      }
   }
   /// Done
   Ok(incomplete_manifests)
}


///
#[hdk_extern]
pub fn check_manifest_integrity(manifest_eh: EntryHash) -> ExternResult<Vec<EntryHash>> {
   debug!("START - {:?}", manifest_eh);
   let manifest = get_typed_from_eh::<ParcelManifest>(manifest_eh.to_owned())?;
   /// Find chunks
   let mut result = Vec::new();
   for chunk_eh in manifest.chunks.clone() {
      let maybe_record = get_local_from_eh(chunk_eh.clone());
      if maybe_record.is_err() {
         debug!("check_manifest_integrity() error: {:?}", maybe_record.err().unwrap());
         result.push(chunk_eh);
      }
   }
   debug!("check_manifest_integrity() result: {:?}", result);
   /// Done
   Ok(result)
}


/// Try to retrieve every chunk
#[hdk_extern]
pub fn fetch_missing_chunks(manifest_eh: EntryHash) -> ExternResult<()> {
   //debug!("START - {:?}", manifest_eh);
   /// Find chunks
   let missing_chunks = check_manifest_integrity(manifest_eh.clone())?;
   if missing_chunks.is_empty() {
      return Ok(());
   }
   /// Private: Find notice
   let _manifest = get_typed_from_eh::<ParcelManifest>(manifest_eh.clone())?;
   let notices = query_DeliveryNotice(DeliveryNoticeQueryField::Parcel(manifest_eh.clone()))?;
   if notices.is_empty() {
      debug!("No Notice found for post-committed ParcelManifest");
      /// Normal if agent is original creator of ParcelManifest
      return Ok(())
   }
   let notice_eh = hash_entry(notices[0].0.clone())?;
   ///
   let mut pairs = Vec::new();
   for chunk_eh in missing_chunks.clone() {
      let input = FetchChunkInput {
         chunk_eh,
         notice_eh: notice_eh.clone(),
      };
      let response = call_self("fetch_chunk", input)?;
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