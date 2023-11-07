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
   let chunks: Vec<EntryHash> = query_all_private_chunks(())?.into_iter()
      .map(|tuple| tuple.0)
      .collect();
   debug!("scan_incomplete_manifests() chunks count: {}", chunks.len());
   let mut incomplete_manifests = Vec::new();
   for (manifest_eh, ts, manifest)  in tuples {
      for chunk_eh in manifest.chunks.clone() {
         if !chunks.contains(&chunk_eh) {
            incomplete_manifests.push(manifest_eh);
            break;
         }
      }
   }
   /// Done
   Ok(incomplete_manifests)
}


///
//#[hdk_extern]
pub fn check_manifest_integrity(manifest_eh: EntryHash, manifest: ParcelManifest) -> ExternResult<Vec<EntryHash>> {
   debug!("check_manifest_integrity() {} {:?}", manifest.description.name, manifest_eh);
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

