use hdk::prelude::*;
use zome_signals::*;
use zome_utils::*;
use zome_delivery_types::*;


///
#[hdk_extern]
pub fn fetch_public_manifest(manifest_eh: EntryHash) -> ExternResult<(ParcelManifest, Timestamp, AgentPubKey)> {
   let (record, manifest) = get_typed_and_record::<ParcelManifest>(AnyLinkableHash::from(manifest_eh), GetStrategy::Local)?; // FIXME GetStrategy
   /// Emit signal
   attest_entry_created(record.clone(), ValidatedBy::Network, false)?;
   ///
   Ok((manifest, record.action().timestamp(), record.action().author().to_owned()))
}


///
#[hdk_extern]
pub fn fetch_chunk(chunk_eh: EntryHash) -> ExternResult<ParcelChunk> {
   //let chunk: ParcelChunk = get_typed_from_eh(chunk_eh)?;
   let (record, chunk) = get_typed_and_record::<ParcelChunk>(AnyLinkableHash::from(chunk_eh), GetStrategy::Local)?; // FIXME GetStrategy
   /// Emit signal
   attest_entry_created(record.clone(), ValidatedBy::Network, false)?;
   ///
   Ok(chunk)
}
