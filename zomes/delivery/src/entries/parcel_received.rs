
use hdk::prelude::*;
use crate::{
   send_item::*,
   entries::*,
};


/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry(id = "ParcelReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub reply_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedHeaderHashed, // signed header of parcel's Element
}

///
pub(crate) fn post_commit_ParcelReceived(
   distribution_eh: &EntryHash,
   recepetion: DeliveryReply,
) -> ExternResult<()>
{
   debug!("post_commit_ParcelReceived() {:?}", distribution_eh);
   /// Create PendingItem
   let pending_item = PendingItem::from_reception(
      recepetion.clone(),
      distribution_eh.clone(),
      recipient.clone(),
   )?;
   /// Send it to recipient
   let res = send_item(
      recipient,
      distribution_eh.clone(),
      pending_item,
      recepetion.sender_description_signature.clone());
   match res {
      Ok(_) => {},
      Err(e) => {
         /// FIXME: accumulate failed recipients to final error return value
         debug!("send_reception_request() failed: {}", e);
      }
   }
   /// Done
   Ok(())
}