use hdk::prelude::*;
use crate::{
   constants::*,
   //EntryKind::ParcelChunk,
   utils::*,
};

#[hdk_entry(id = "ParcelManifest", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelManifest {
   pub name: String,
   pub app_type: String,
   //pub data_hash: String,
   pub size: usize,
   pub chunks: Vec<EntryHash>,
}

impl ParcelManifest {
   pub fn validate(&self) -> Result<(), String> {
      /// Must have chunks
      if self.chunks.is_empty() {
         return Err("Missing chunks".to_owned());
      }
      /// Must not exceed size limit
      if self.size > PARCEL_MAX_SIZE {
         return Err(format!("Parcel is too big: {} > {}", self.size, PARCEL_MAX_SIZE));
      }
      /// Must meet minimum name length
      if self.name.len() < NAME_MIN_LENGTH {
         return Err(format!("Name is too small: {} > {}", self.name, NAME_MIN_LENGTH));
      }

      /// FIXME: Check each entry exists and is a ParcelChunk
      /// FIXME: Also check total size

      /// Done
      Ok(())
   }


   /// Try to retrieve every chunk
   pub fn post_commit(manifest_eh: &EntryHash, manifest: Self) -> ExternResult<()> {
      debug!("post_commit_ParcelManifest() {:?}", manifest_eh);
      /// Try to retrieve parcel if it has been accepted
      for chunk_eh in manifest.chunks {
         let response = call_self("fetch_chunk", chunk_eh)?;
         debug!("fetch_chunk() response: {:?}", response);
         //assert!(matches!(response, ZomeCallResponse::Ok { .. }));

      }
      /// Done
      Ok(())
   }
}
