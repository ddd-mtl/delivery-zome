use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Grab all chunks in source-chain. Check if each manifest chunk is found in that list.
#[hdk_extern]
pub fn determine_missing_chunks(manifest_eh: EntryHash) -> ExternResult<Vec<EntryHash>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("START - {}",manifest_eh);
   let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
   debug!("manifest: {}", manifest.description.name);
   let chunks: Vec<EntryHash> = query_all_private_chunks(())?
      .into_iter()
      .map(|tuple| tuple.0)
      .collect();
   /// Find chunks
   let mut missing_chunks = Vec::new();
   for chunk_eh in manifest.chunks.clone() {
      if !chunks.contains(&chunk_eh) {
         missing_chunks.push(chunk_eh);
      }
   }
   debug!("missing_chunks len = {}", missing_chunks.len());
   /// Done
   Ok(missing_chunks)
}
