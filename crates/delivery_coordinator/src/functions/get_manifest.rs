use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;


///
#[hdk_extern]
pub fn get_manifest(manifest_eh: EntryHash) -> ExternResult<(ParcelManifest, Timestamp, AgentPubKey)> {
   let (record, manifest) = get_typed_and_record::<ParcelManifest>(&AnyLinkableHash::from(manifest_eh))?;
   Ok((manifest, record.action().timestamp(), record.action().author().to_owned()))
}


///
#[hdk_extern]
pub fn get_chunk(chunk_eh: EntryHash) -> ExternResult<ParcelChunk> {
   let chunk: ParcelChunk = get_typed_from_eh(chunk_eh)?;
   Ok(chunk)
}
