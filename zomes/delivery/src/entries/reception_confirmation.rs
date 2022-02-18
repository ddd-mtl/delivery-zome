
use hdk::prelude::*;

#[hdk_entry(id = "ReceptionConfirmation")]
#[derive(Clone, PartialEq)]
pub struct ReceptionConfirmation {
   pub recipient_signature: Signautre,
   pub parcel_accepted: bool,
   pub manifest_delivery_hh: HeaderHash,
   pub maybe_parcel_hh: HeaderHash,
}