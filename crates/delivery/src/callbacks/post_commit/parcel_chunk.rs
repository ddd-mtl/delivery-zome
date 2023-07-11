use hdk::prelude::*;
use zome_delivery_integrity::*;
use zome_utils::*;


///
pub fn post_commit_ParcelChunk(entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_ParcelChunk() {:?}", chunk_eh);
    let _ = ParcelChunk::try_from(entry)?;
    /// Create ParcelReceived if we fetched all chunks
    let response = call_self("check_manifest", chunk_eh)?;
    debug!("check_manifest() response: {:?}", response);
    assert!(matches!(response, ZomeCallResponse::Ok { .. }));
    Ok(())
}