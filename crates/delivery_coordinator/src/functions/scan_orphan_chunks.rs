use hdk::prelude::*;
use zome_utils::*;
//use zome_delivery_types::*;
use crate::*;
use crate::functions::get_all_manifests::*;


///
/// Return list of chunks with no local Manifest (Public, Private)
#[hdk_extern]
fn scan_orphan_chunks(_ : ()) -> ExternResult<(Vec<EntryHash>, Vec<EntryHash>)> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Public
   let mut public_orphans = Vec::new();
   let manifests = get_all_local_public_manifests(())?;
   let known_chunks: Vec<EntryHash> = manifests.into_iter()
      .map(|(_eh, manifest)| manifest.chunks)
      .flatten()
      .collect();
   debug!("known public chunks: {}", known_chunks.len());
   let found_chunks = query_all_public_chunks(())?;
   debug!("found public chunks: {}", found_chunks.len());
   for found_chunk in found_chunks {
      let index = known_chunks.iter().position(|x| *x == found_chunk.0);
      if index.is_none() {
         public_orphans.push(found_chunk.0.to_owned());
      }
   }

   /// Private
   let mut private_orphans = Vec::new();
   let manifests = get_all_private_manifests(())?;
   let known_chunks: Vec<EntryHash> = manifests.into_iter()
                                                   .map(|(_eh, manifest)| manifest.chunks)
                                                   .flatten()
                                                   .collect();
   debug!("known private chunks: {}", known_chunks.len());
   let found_chunks = query_all_private_chunks(())?;
   debug!("found private chunks: {}", found_chunks.len());
   for found_chunk in found_chunks {
      let index = known_chunks.iter().position(|x| *x == found_chunk.0);
      if index.is_none() {
         private_orphans.push(found_chunk.0.to_owned());
      }
   }

   /// Done
   Ok((public_orphans, private_orphans))
}