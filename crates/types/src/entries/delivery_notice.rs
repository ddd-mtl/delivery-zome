use hdk::prelude::*;

use crate::parcel::*;

/// Entry representing a received Manifest
#[hdk_entry(id = "DeliveryNotice", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
    pub distribution_eh: EntryHash,
    pub sender: AgentPubKey,
    pub sender_summary_signature: Signature,
    pub parcel_summary: ParcelSummary,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeliveryNoticeQueryField {
    Sender(AgentPubKey),
    Parcel(EntryHash)
}




