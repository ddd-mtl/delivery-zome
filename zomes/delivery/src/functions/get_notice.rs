use hdk::prelude::*;

use crate::{entries::*, utils::*, send_dm, dm_protocol::*, link_kind::*, ParcelReference, EntryKind};


/// Zone Function
/// Return DeliveryNotice from which we received a Parcel
#[hdk_extern]
pub fn get_notice(parcel_eh: EntryHash) -> ExternResult<Option<DeliveryNotice>> {
   let field = ParcelReceivedField::Parcel(parcel_eh.clone());
   let maybe_receipt = ParcelReceived::query(field)?;
   if maybe_receipt.is_none() {
      return Ok(None)
   }
   let notice_eh = maybe_receipt.unwrap().notice_eh;
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   /// Done
   Ok(Some(notice))
}