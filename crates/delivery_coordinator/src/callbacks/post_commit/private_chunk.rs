use hdk::prelude::*;
use zome_delivery_types::*;


///
pub fn post_commit_PrivateChunk(_sah: &SignedActionHashed, entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_ParcelChunk() {:?}", chunk_eh);
    let chunk = ParcelChunk::try_from(entry)?;
    /// Emit signal
    let res = emit_signal(&SignalProtocol::NewLocalChunk((chunk_eh.to_owned(), chunk.clone())));
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    Ok(())
}
