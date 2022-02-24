use hdk::prelude::*;

//use crate::entries::*;
//use crate::entries::pub_enc_key::*;
use crate::utils::*;
use crate::entry_kind::*;
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




