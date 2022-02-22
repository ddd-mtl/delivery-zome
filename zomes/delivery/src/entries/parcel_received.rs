
use hdk::prelude::*;
use crate::{
   send_item::*,
   entries::*,
   utils::*,
};


/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry(id = "ParcelReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedHeaderHashed, // signed header of parcel's Element
}

///
pub(crate) fn post_commit_ParcelReceived(
   _reception_eh: &EntryHash,
   reception: ParcelReceived,
) -> ExternResult<()>
{
   debug!("post_commit_ParcelReceived() {:?}", distribution_eh);
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(reception.notice_eh.clone())?;
   /// Sign Item
   let signature = sign(agent_info()?.agent_latest_pubkey, reception.clone())?;
   /// Create PendingItem
   let pending_item = PendingItem::from_reception(
      reception.clone(),
      notice.distribution_eh.clone(),
      notice.sender.clone(),
   )?;
   /// Send it to recipient
   let _ = send_item(
      notice.sender,
      notice.distribution_eh.clone(),
      pending_item,
      signature)?;
   /// Done
   Ok(())
}