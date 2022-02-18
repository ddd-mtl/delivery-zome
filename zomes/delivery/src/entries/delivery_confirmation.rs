use hdk::prelude::*;

use crate::entries::ReceptionResponse;

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryConfirmation", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryConfirmation {
    pub distribution_eh: EntryHash,
    pub recipient: AgentPubKey,
    pub recipient_response: ReceptionResponse,
    pub date_of_response: u64,
}
