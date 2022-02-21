use hdk::prelude::*;
use crate::{
   constants::*,
   EntryKind::ParcelChunk,
};
use crate::entries::*;

#[hdk_entry(id = "ParcelManifest", visibility = "private")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParcelManifest {
   pub name: String,
   pub app_type: AppType,
   //pub data_hash: String,
   pub size: usize,
   pub chunks: Vec<EntryHash>,
}

pub fn validate_ParcelManifest(input: ParcelManifest) -> Result<(), String> {
   /// Must have chunks
   if input.chunks.is_empty() {
      return Err("Missing chunks".to_owned());
   }
   /// Must not exceed size limit
   if input.size > PARCEL_MAX_SIZE {
      return Err(format!("Parcel is too big: {} > {}", input.size, PARCEL_MAX_SIZE));
   }
   /// Must meet minimum name length
   if input.name < NAME_MIN_LENGTH {
      return Err(format!("Name is too small: {} > {}", input.name, NAME_MIN_LENGTH));
   }

   /// FIXME: Check each entry exists and is a ParcelChunk
   /// FIXME: Also check total size

   /// Done
   Ok(())
}
