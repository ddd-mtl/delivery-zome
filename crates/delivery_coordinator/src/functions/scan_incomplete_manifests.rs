use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::DeliveryEntryTypes;
use zome_delivery_types::{ParcelChunk, ParcelManifest};


/// Check if each private manifest is complete
/// Return EntryHash of every incomplete manifests
#[hdk_extern]
pub fn scan_incomplete_manifests(_: ()) -> ExternResult<Vec<EntryHash>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let tuples = get_all_typed_local::<ParcelManifest>(DeliveryEntryTypes::PrivateManifest.try_into().unwrap())?;
   debug!("scan_incomplete_manifests() manifests count: {}", tuples.len());
   let entry_type = DeliveryEntryTypes::PrivateChunk.try_into().unwrap();
   debug!("PrivateChunk entry_type: {:?}", entry_type);
   let chunks: Vec<EntryHash> = get_all_typed_local::<ParcelChunk>(entry_type)?
     .into_iter()
     .map(|(_ah, create, _typed)| create.entry_hash)
     .collect();
   debug!("scan_incomplete_manifests() chunks count: {}", chunks.len());
   let mut incomplete_manifests = Vec::new();
   for (_ah, create, manifest)  in tuples {
      for chunk_eh in manifest.chunks.clone() {
         if !chunks.contains(&chunk_eh) {
            incomplete_manifests.push(create.entry_hash);
            break;
         }
      }
   }
   /// Done
   Ok(incomplete_manifests)
}

