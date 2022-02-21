use hdk::prelude::*;
use crate::{
   constants::PARCEL_MAX_SIZE,
   EntryKind::ParcelChunk,
   entries::*,
};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitParcelManifestInput {
   pub name: String,
   pub app_type: AppType,
   pub chunks: Vec<EntryHash>,
}


/// Zome function
/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_manifest(input: CommitParcelManifestInput) -> ExternResult<EntryHash> {
   trace!(" commit_parcel_manifest({}) -  {}", input.parcel_type, input.name);
   /// Create entry
   let size = match input.parcel_content {
      Parcel::AppEntry(eh) => get_app_entry_size(eh)?,
      Parcel::Chunks(chunks) => {
         let mut total_parcel_size = 0;
         for chunk_eh in chunks {
            total_parcel_size += get_app_entry_size(chunk_eh)?;
         }
         total_parcel_size
      }
   };
   let manifest = ParcelManifest {
      name: input.name,
      app_type: input.app_type,
      //data_hash:
      size,
      chunks: input.chunks,
   };
   /// Commit entry
   let manifest_eh = hash_entry(manifest)?;
   let _ = create_entry(&manifest)?;
   /// Done
   Ok(manifest_eh)
}
