use hdk::prelude::*;
use zome_delivery_integrity::*;
use zome_utils::*;
use zome_delivery_types::*;

use crate::send_item::*;
use crate::functions::*;

///
pub fn post_commit_ParcelReceived(entry: Entry, receipt_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_ParcelReceived() {:?}", receipt_eh);
   let parcel_received = ParcelReceived::try_from(entry)?;
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(parcel_received.notice_eh.clone())?;
   /// Create PendingItem
   let pending_item = pack_reception(
      parcel_received.clone(),
      notice.distribution_eh.clone(),
      notice.sender.clone(),
   )?;
   /// Send it to recipient
   let _ = send_item(
      notice.sender,
      pending_item,
      notice.summary.distribution_strategy,
   )?;
   /// Done
   Ok(())
}
