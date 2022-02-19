use std::future::Pending;
use hdk::prelude::*;
use crate::{
   get_typed_from_eh, entries::*, utils_parcel::*, LinkKind, utils::*, parcel::*, dm::*, DeliveryProtocol,
};
use crate::EntryKind::ReceptionConfirmation;
use hc_utils::*;

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
#[snapmail_api]
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