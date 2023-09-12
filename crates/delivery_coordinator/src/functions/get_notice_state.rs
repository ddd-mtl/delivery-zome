use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::*;


///
#[hdk_extern]
pub fn get_notice_state(notice_eh: EntryHash) -> ExternResult<(NoticeState, usize)> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("START");
   /// Make sure EntryHash is correct
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// look for reply
   let maybe_reply = query_NoticeReply(notice_eh.clone())?;
   if maybe_reply.is_none() {
      return Ok((NoticeState::Unreplied, 0));
   }
   if !maybe_reply.unwrap().has_accepted {
      return Ok((NoticeState::Refused, 0));
   }
   /// Look for parcel
   //let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   //let has_parcel = has_parcel(notice.summary.parcel_reference)?;
   let maybe_parcel = query_ReceptionProof(ReceptionProofQueryField::Notice(notice_eh.clone()))?;
   /// Done
   if maybe_parcel.is_some() {
      return Ok((NoticeState::Received, 0));
   }
   /// Count chunks if it has a manifest
   let mut pct = 0;
   if let ParcelReference::Manifest(mref) = notice.summary.parcel_reference {
      pct = count_chunks_received(mref.manifest_eh)?;
   }
   /// Done
   Ok((NoticeState::Accepted, pct))
}
