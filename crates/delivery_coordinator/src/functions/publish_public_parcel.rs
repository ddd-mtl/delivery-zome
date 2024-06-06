use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;
use crate::determine_parcel_size;


///
#[hdk_extern]
#[feature(zits_blocking = "PublicParcel")]
pub fn publish_public_parcel(manifest_arg: ParcelManifest) -> ExternResult<EntryHash> {
   debug!(" START - {}", manifest_arg.description.name);
   std::panic::set_hook(Box::new(zome_panic_hook));
   if manifest_arg.chunks.is_empty() {
      return error("No chunks in Manifest");
   }
   /// Compute size
   let mut manifest = manifest_arg.clone();
   let computed_size = determine_parcel_size(manifest_arg.clone())?;
   if manifest.description.size == 0 {
      manifest.description.size = computed_size;
   }
   debug!(" Size : {} == {}?", manifest.description.size, computed_size);
   /// Commit PublicManifest entry
   let manifest_eh = hash_entry(manifest.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::PublicManifest(manifest.clone()))?;

   /// Create Description
   let pr = ParcelReference {
      parcel_eh: manifest_eh,
      description: manifest.description,
   };
   /// Commit PublicParcel entry
   let pp_eh = hash_entry(pr.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::PublicParcel(pr))?;
   /// Done
   Ok(pp_eh)
}
