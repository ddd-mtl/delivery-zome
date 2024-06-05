use hdk::prelude::*;
use zome_delivery_types::*;
use crate::emit_self_signal;


///
pub fn post_commit_PublicChunk(_sah: &SignedActionHashed, entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicChunk() {:?}", chunk_eh);
   let chunk = ParcelChunk::try_from(entry)?;
   /// Emit signal
   let res = emit_self_signal(DeliverySignalProtocol::NewLocalChunk((chunk_eh.to_owned(), chunk.clone())));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}
