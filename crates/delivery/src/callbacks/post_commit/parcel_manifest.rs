use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_integrity::*;
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
      let output: FetchChunkOutput = decode_response(response)?;
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


//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub type CommitChunksInput = Vec<(ParcelChunk, Option<Link>)>;

/// Internal Zome function
#[hdk_extern]
fn commit_chunks(chunks: CommitChunksInput) -> ExternResult<()> {
   debug!("commit_chunks() len = {}", chunks.len());
   /// Check if chunk not already committed
   let mut chunk_ehs: Vec<EntryHash> = chunks.iter().map(|pair| {
      hash_entry(pair.0.clone()).unwrap()
   } ).collect();
   let set: HashSet<_> = chunk_ehs.drain(..).collect(); // dedup
   let query_args = ChainQueryFilter::default()
      .include_entries(false)
      .entry_hashes(set)
      ;
   let locals = query(query_args)?;
   let local_ehs: Vec<EntryHash> = locals.iter()
      .map(|x| x.action().entry_hash().unwrap().to_owned())
      .collect();
   for (chunky, maybe_link) in chunks.into_iter() {
      let chunk: ParcelChunk = chunky;
      /// Skip local
      let chunk_eh = hash_entry(chunk.clone())?;
      if local_ehs.contains(&chunk_eh) {
         continue;
      }
      /// Commit Parcel
      let _ = create_entry_relaxed(DeliveryEntry::ParcelChunk(chunk))?;

      /// Delete Link
      if let Some(link) = maybe_link {
         let _ = delete_link(link.create_link_hash)?;
      }
   }
   /// Done
   Ok(())
}