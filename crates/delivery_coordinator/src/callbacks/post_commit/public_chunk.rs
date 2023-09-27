use hdk::prelude::*;
use zome_delivery_types::*;


///
pub fn post_commit_PublicChunk(_sah: &SignedActionHashed, entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicChunk() {:?}", chunk_eh);
   let _ = ParcelChunk::try_from(entry)?;
   /// Done
   Ok(())
}