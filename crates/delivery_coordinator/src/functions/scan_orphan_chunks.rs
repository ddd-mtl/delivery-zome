use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::DeliveryEntryTypes;
use zome_delivery_types::{ParcelChunk, ParcelManifest};
//use zome_delivery_types::*;
//use crate::*;


///
/// Return list of chunks with no local Manifest (Public, Private)
#[hdk_extern]
fn scan_orphan_chunks(_ : ()) -> ExternResult<(Vec<EntryHash>, Vec<EntryHash>)> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Public
   let mut public_orphans = Vec::new();
   let manifests: Vec<ParcelManifest> = get_all_typed_local::<ParcelManifest>(DeliveryEntryTypes::PublicManifest.try_into().unwrap())?
     .into_iter()
     .map(|(_ah, _create, typed)| typed)
     .collect();
   let known_chunks: Vec<EntryHash> = manifests.into_iter()
      .map(|manifest| manifest.chunks)
      .flatten()
      .collect();
   debug!("known public chunks: {}", known_chunks.len());
   let found_chunks = get_all_typed_local::<ParcelChunk>(DeliveryEntryTypes::PublicChunk.try_into().unwrap())?;
   debug!("found public chunks: {}", found_chunks.len());
   for (_ah, create, _chunk) in found_chunks {
      let index = known_chunks.iter().position(|x| *x == create.entry_hash);
      if index.is_none() {
         public_orphans.push(create.entry_hash.to_owned());
      }
   }

   /// Private
   let mut private_orphans = Vec::new();
   let manifests: Vec<ParcelManifest> = get_all_typed_local::<ParcelManifest>(DeliveryEntryTypes::PrivateManifest.try_into().unwrap())?
     .into_iter()
     .map(|(_ah, _create, typed)| typed)
     .collect();
   let known_chunks: Vec<EntryHash> = manifests.into_iter()
                                                   .map(|manifest| manifest.chunks)
                                                   .flatten()
                                                   .collect();
   debug!("known private chunks: {}", known_chunks.len());
   let found_chunks = get_all_typed_local::<ParcelChunk>(DeliveryEntryTypes::PrivateChunk.try_into().unwrap())?;
   debug!("found private chunks: {}", found_chunks.len());
   for (_ah, create, _chunk) in found_chunks {
      let index = known_chunks.iter().position(|x| *x == create.entry_hash);
      if index.is_none() {
         private_orphans.push(create.entry_hash.to_owned());
      }
   }
   debug!("orphans: {} {}", public_orphans.len(), private_orphans.len());
   /// Done
   Ok((public_orphans, private_orphans))
}
