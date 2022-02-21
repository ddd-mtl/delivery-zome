use hdk::prelude::*;

use crate::entries::ReceptionResponse;

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryReceipt", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryReceipt {
    pub distribution_eh: EntryHash,
    pub recipient: AgentPubKey,
    pub recipient_signature: Signature,
    //pub date_of_reception: u64,
}
