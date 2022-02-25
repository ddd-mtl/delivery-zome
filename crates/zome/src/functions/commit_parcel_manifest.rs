use hdk::prelude::*;
use zome_delivery_types::*;


/// Zome function
/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_manifest(input: ParcelManifest) -> ExternResult<EntryHash> {
   trace!(" commit_parcel_manifest({}) -  {}", input.custum_entry_type, input.name);
   /// Commit entry
   let manifest_eh = hash_entry(input.clone())?;
   let _ = create_entry(&input)?;
   /// Done
   Ok(manifest_eh)
}
