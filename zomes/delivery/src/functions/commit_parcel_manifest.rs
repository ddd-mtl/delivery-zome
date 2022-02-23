use hdk::prelude::*;
use delivery_zome_api::{entries::*, entry_kind::*, parcel::*, utils::*};


/// Zome function
/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_manifest(input: ParcelManifest) -> ExternResult<EntryHash> {
   trace!(" commit_parcel_manifest({}) -  {}", input.entry_id, input.name);
   /// Commit entry
   let manifest_eh = hash_entry(input.clone())?;
   let _ = create_entry(&input)?;
   /// Done
   Ok(manifest_eh)
}