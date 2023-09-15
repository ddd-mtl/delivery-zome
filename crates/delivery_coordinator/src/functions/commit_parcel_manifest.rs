use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;
use crate::get_app_entry_size;


/// Return EntryHash of newly created ParcelManifest
#[hdk_extern]
pub fn commit_parcel_manifest(manifest_arg: ParcelManifest) -> ExternResult<EntryHash> {
   trace!(" START - {}", manifest_arg.data_hash);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Compute size
   let mut manifest = manifest_arg.clone();
   let computed_size = determine_parcel_size(manifest_arg.clone())?;
   if manifest.description.size == 0 {
      manifest.description.size = computed_size;
   }
   debug!(" Size : {} == {}?", manifest.description.size, computed_size);
   /// Commit entry
   let manifest_eh = hash_entry(manifest.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::ParcelManifest(manifest))?;
   /// Done
   Ok(manifest_eh)
}




///
pub fn determine_parcel_size(manifest: ParcelManifest) -> ExternResult<u64> {
   //let last_chunk: ParcelChunk = get_typed_from_eh(input.manifest.chunks.last().unwrap().to_owned())?;
   let last_chunk_size = get_app_entry_size(manifest.chunks.last().unwrap().to_owned())?;
   let size: u64 = (manifest.chunks.len() as u64 - 1) * get_dna_properties().max_chunk_size as u64 + last_chunk_size as u64;
   Ok(size)
}