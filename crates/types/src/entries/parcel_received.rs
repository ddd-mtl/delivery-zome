use hdk::prelude::*;

/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry(id = "ParcelReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedHeaderHashed, // signed header of parcel's Element
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParcelReceivedQueryField {
   Notice(EntryHash),
   Parcel(EntryHash)
}
