use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;


///
#[hdk_extern]
pub fn get_notice_state(notice_eh: EntryHash) -> ExternResult<NoticeState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_notice_state() CALLED");
   /// look for reply
   let maybe_reply = query_DeliveryReply(notice_eh.clone())?;
   if maybe_reply.is_none() {
      return Ok(NoticeState::Unreplied);
   }
   if !maybe_reply.unwrap().has_accepted {
      return Ok(NoticeState::Refused);
   }
   /// Look for parcel
   //let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   //let has_parcel = has_parcel(notice.summary.parcel_reference)?;
   let maybe_parcel = query_ParcelReceived(ParcelReceivedQueryField::Notice(notice_eh.clone()))?;
   /// Done
   if maybe_parcel.is_some() {
      return Ok(NoticeState::Received)
   }
   Ok(NoticeState::Accepted)
}
