use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;

///
pub fn post_commit_ParcelReceived(entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_ParcelReceived() {:?}", eh);
   let parcel_received = ParcelReceived::try_from(entry)?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::ReceivedParcel(parcel_received.clone()));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(parcel_received.notice_eh.clone())?;
   /// Create PendingItem
   let pending_item = pack_reception(
      parcel_received.clone(),
      notice.distribution_eh.clone(),
      notice.sender.clone(),
   )?;
   /// Send it to sender
   let _ = send_item(
      notice.sender,
      pending_item,
      notice.summary.distribution_strategy,
   )?;
   /// Done
   Ok(())
}
