use hdk::prelude::*;
use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_DeliveryReceipt(entry: Entry, _eh: &EntryHash) -> ExternResult<()> {
   let receipt = DeliveryReceipt::try_from(entry)?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::ReceivedReceipt(receipt));
   if let Err(err) = res.clone() {
      error!("Emit signal failed: {}", err);
   } else {
      debug!("Emit signal successful!");
   }
   /// Done
   Ok(())
}