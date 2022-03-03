use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

use crate::zome_entry_trait::*;
use crate::constants::*;
use crate::functions::*;


impl ZomeEntry for ParcelManifest {
   fn validate(&self, _maybe_package: Option<ValidationPackage>) -> ExternResult<ValidateCallbackResult> {
      /// Must have chunks
      if self.chunks.is_empty() {
         return invalid("Missing chunks");
      }
      /// Must not exceed size limit
      if self.size > PARCEL_MAX_SIZE {
         return invalid(&format!("Parcel is too big: {} > {}", self.size, PARCEL_MAX_SIZE));
      }
      /// Must meet minimum name length
      if self.name.len() < NAME_MIN_LENGTH {
         return invalid(&format!("Name is too small: {} > {}", self.name, NAME_MIN_LENGTH));
      }
      /// FIXME: Check each entry exists and is a ParcelChunk
      /// FIXME: Also check total size
      /// Done
      Ok(ValidateCallbackResult::Valid)
   }

   /// Try to retrieve every chunk
   fn post_commit(&self, manifest_eh: &EntryHash) -> ExternResult<()> {
      debug!("post_commit_ParcelManifest() {:?}", manifest_eh);
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
      for chunk_eh in self.chunks.clone() {
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
      .map(|x| x.header().entry_hash().unwrap().to_owned())
      .collect();
   for (chunk, maybe_link) in chunks {
      /// Skip local
      let chunk_eh = hash_entry(chunk.clone())?;
      if local_ehs.contains(&chunk_eh) {
         continue;
      }
      /// Commit Parcel
      let _hh = create_entry(chunk)?;
      /// Delete Link
      if let Some(link) = maybe_link {
         let _hh = delete_link(link.create_link_hash)?;
      }
   }
   /// Done
   Ok(())
}