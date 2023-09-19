use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;


///
#[hdk_extern]
pub fn get_manifest(manifest_eh: EntryHash) -> ExternResult<ParcelManifest> {
   let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
   Ok(manifest)
}


///
#[hdk_extern]
pub fn get_chunk(chunk_eh: EntryHash) -> ExternResult<ParcelChunk> {
   let chunk: ParcelChunk = get_typed_from_eh(chunk_eh)?;
   Ok(chunk)
}