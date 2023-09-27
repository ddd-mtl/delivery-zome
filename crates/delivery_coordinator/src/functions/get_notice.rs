use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;

use crate::*;


/// Return DeliveryNotice (& state) from which we received a Parcel
#[hdk_extern]
pub fn get_notice(distribution_ah: ActionHash) -> ExternResult<Option<GetNoticeOutput>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_notice()");
   let field = DeliveryNoticeQueryField::Distribution(distribution_ah.clone());
   let maybe_notices = query_DeliveryNotice(field)?;
   if maybe_notices.is_empty() {
      return Ok(None)
   }
   let notice_eh = hash_entry(maybe_notices[0].0.clone())?;
   /// Get state
   let output = GetNoticeOutput {
      notice: maybe_notices[0].0.clone(),
      state: get_notice_state(notice_eh)?,
   };
   /// Done
   Ok(Some(output))
}


/// Return DeliveryNotice from which we received a Parcel
#[hdk_extern]
pub fn get_parcel_notice(parcel_eh: EntryHash) -> ExternResult<Option<DeliveryNotice>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_parcel_notice()");
   let field = ReceptionProofQueryField::Parcel(parcel_eh.clone());
   let maybe_reception = query_ReceptionProof(field)?;
   if maybe_reception.is_none() {
      return Ok(None)
   }
   let notice_eh = maybe_reception.unwrap().2.notice_eh;
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// Done
   Ok(Some(notice))
}