use hdk::prelude::*;
use crate::{
   get_typed_from_eh,
   entries::*,
   utils_parcel::*,
};
use crate::EntryKind::ReceptionConfirmation;


/// Zone Function
#[hdk_extern]
#[snapmail_api]
pub fn accept_parcel(notification_eh: EntryHash) -> ExternResult<EntryHash> {
   /// Get DeliveryNotification
   let notification: DeliveryNotification = get_typed_from_eh(notification_eh)?;
   /// Sign?
   // FIXME

   /// Request Parcel
   // FIXME
}
