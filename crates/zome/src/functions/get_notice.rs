use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;

use crate::functions::query::*;


/// Zone Function
/// Return DeliveryNotice from which we received a Parcel
#[hdk_extern]
pub fn get_notice(parcel_eh: EntryHash) -> ExternResult<Option<DeliveryNotice>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let field = ParcelReceivedQueryField::Parcel(parcel_eh.clone());
   let maybe_receipt = query_ParcelReceived(field)?;
   if maybe_receipt.is_none() {
      return Ok(None)
   }
   let notice_eh = maybe_receipt.unwrap().notice_eh;
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   /// Done
   Ok(Some(notice))
}