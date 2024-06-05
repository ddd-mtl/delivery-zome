use hdk::prelude::*;
use zome_delivery_types::*;
use crate::emit_self_signal;


///
pub fn post_commit_ReceptionAck(sah: &SignedActionHashed, entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   let reception_ack = ReceptionAck::try_from(entry)?;
   /// Emit Signal
   let res = emit_self_signal(SignalProtocol::NewReceptionAck((eh.to_owned(), sah.hashed.content.timestamp(), reception_ack)));
   if let Err(err) = res.clone() {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}
