//! All Zome function input types

use hdk::prelude::*;

use crate::delivery::*;
use crate::DeliveryNotice;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeParcelInput {
   pub recipients: Vec<AgentPubKey>,
   pub strategy: DistributionStrategy,
   pub parcel_ref: ParcelReference,
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RespondToNoticeInput {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FetchChunkInput {
   pub chunk_eh: EntryHash,
   pub notice_eh: EntryHash,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNoticeOutput {
   pub notice: DeliveryNotice,
   pub state: NoticeState,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeliveryNoticeQueryField {
   Sender(AgentPubKey),
   Distribution(EntryHash),
   Parcel(EntryHash)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParcelReceivedQueryField {
   Notice(EntryHash),
   Parcel(EntryHash)
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoticeReceivedQueryField {
   Recipient(AgentPubKey),
   Distribution(EntryHash)
}
