use std::future::Pending;
use hdk::prelude::*;
use crate::{
   utils::*, entries::*, utils_parcel::*, LinkKind, utils::*, parcel::*, dm::*, DeliveryProtocol,
};
use crate::EntryKind::ReceptionConfirmation;

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn refuse_parcel(notification_eh: EntryHash) -> ExternResult<()> {
   /// TODO: Make sure we do not have a Confirmation already for this notification
   //FIXME
   /// Create ReceptionConfirmation
   let confirmation = ReceptionConfirmation {
      notification_eh,
      reception_response: ReceptionResponse::Refused,
   };
   let _hh = create_entry(confirmation)?;
   /// Done
   Ok(())
}