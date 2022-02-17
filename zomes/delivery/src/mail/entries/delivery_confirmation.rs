use hdk::prelude::*;

/// Entry for confirming a delivery has been well received by a recipient
#[hdk_entry(id = "delivery_confirmation", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryConfirmation {
    pub distribution_eh: EntryHash,
    pub recipient: AgentPubKey,
    pub recipient_parcel_signature: Signature,
    pub date_of_reception: u64,
    pub accepted_parcel: bool,
}
