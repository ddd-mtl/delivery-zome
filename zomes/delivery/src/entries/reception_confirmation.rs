
use hdk::prelude::*;
use crate::{
   send_item::*,
   entries::*,
};

pub enum ReceptionResponse {
   // HeaderHash of Parcel stored on recipeint's chain
   // Recipient's signature of parcel entry
   Accepted((HeaderHash, Signature)),
   Refused,
}

#[hdk_entry(id = "ReceptionConfirmation")]
#[derive(Clone, PartialEq)]
pub struct ReceptionConfirmation {
   pub notification_eh: EntryHash,
   pub reception_response: ReceptionResponse,
}



///
pub(crate) fn post_commit_reception(distribution_eh: &EntryHash, recepetion: ReceptionConfirmation) -> ExternResult<()> {
   debug!("post_commit_reception() {:?}", distribution_eh);

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