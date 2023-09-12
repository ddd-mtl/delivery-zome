use hdk::prelude::*;
use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_ReceptionAck(entry: Entry, _eh: &EntryHash) -> ExternResult<()> {
   let receipt = ReceptionAck::try_from(entry)?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::ReceivedReceptionAck(receipt));
   if let Err(err) = res.clone() {
      error!("Emit signal failed: {}", err);
   } else {
      debug!("Emit signal successful!");
   }
   /// Done
   Ok(())
}