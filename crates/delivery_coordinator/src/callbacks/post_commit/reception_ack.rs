use hdk::prelude::*;
use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_ReceptionAck(entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   let reception_ack = ReceptionAck::try_from(entry)?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::NewReceptionAck((eh.to_owned(), reception_ack)));
   if let Err(err) = res.clone() {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}