use hdk::prelude::*;

pub enum ReceptionResponse {
    Accepted((HeaderHash, Signature)),
    Refused,
}

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryReply", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryReply {
    pub distribution_eh: EntryHash,
    pub has_accepted: bool,
}
