use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Check if each private manifest is complete
/// Return EntryHash of every incomplete manifests
#[hdk_extern]
pub fn scan_incomplete_manifests(_: ()) -> ExternResult<Vec<EntryHash>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let tuples = query_all_private_manifests(())?;
   debug!("scan_incomplete_manifests() manifests count: {}", tuples.len());
   let mut incomplete_manifests = Vec::new();
   for tuple in tuples {
      let chunks = check_manifest_integrity(tuple.0.clone())?;
      if !chunks.is_empty() {
         incomplete_manifests.push(tuple.0);
      }
   }
   /// Done
   Ok(incomplete_manifests)
}


///
#[hdk_extern]
pub fn check_manifest_integrity(manifest_eh: EntryHash) -> ExternResult<Vec<EntryHash>> {
   debug!("checking manifest {:?}", manifest_eh);
   let manifest = get_typed_from_eh::<ParcelManifest>(manifest_eh.to_owned())?;
   debug!("checking manifest {}", manifest.description.name);
   /// Find chunks
   let mut result = Vec::new();
   for chunk_eh in manifest.chunks.clone() {
      let maybe_record = get_local_from_eh(chunk_eh.clone());
      if maybe_record.is_err() {
         //debug!("check_manifest_integrity() error: {:?}", maybe_record.err().unwrap());
         result.push(chunk_eh);
      }
   }
   debug!("missing: {}", result.len());
   /// Done
   Ok(result)
}

