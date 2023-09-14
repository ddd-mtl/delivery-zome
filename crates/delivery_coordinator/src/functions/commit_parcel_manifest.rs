use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;

/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_manifest(manifest: ParcelManifest) -> ExternResult<EntryHash> {
   trace!(" START - {}", manifest.data_hash);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Commit entry
   let manifest_eh = hash_entry(manifest.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::ParcelManifest(manifest))?;
   /// Done
   Ok(manifest_eh)
}
