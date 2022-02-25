use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use crate::utils::*;
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
      for chunk_eh in self.chunks.clone() {
         let input = FetchChunkInput {
            chunk_eh,
            notice_eh: notice_eh.clone(),
         };
         let response = call_self("fetch_chunk", input)?;
         debug!("fetch_chunk() response: {:?}", response);
         assert!(matches!(response, ZomeCallResponse::Ok { .. }));
      }
      /// Done
      Ok(())
   }
}