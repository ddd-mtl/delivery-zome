//! All Zome function input types

use hdi::prelude::*;

use crate::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeParcelInput {
   pub recipients: Vec<AgentPubKey>,
   pub strategy: DistributionStrategy,
   pub parcel_ref: ParcelReference,
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespondToNoticeInput {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchChunkInput {
   pub chunk_eh: EntryHash,
   pub notice_eh: EntryHash,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNoticeOutput {
   pub notice: DeliveryNotice,
   pub state: NoticeState,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeliveryNoticeQueryField {
   Sender(AgentPubKey),
   Distribution(EntryHash),
   Parcel(EntryHash)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParcelReceivedQueryField {
   Notice(EntryHash),
   Parcel(EntryHash)
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoticeReceivedQueryField {
   Recipient(AgentPubKey),
   Distribution(EntryHash)
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitPendingItemInput {
   pub item: PendingItem,
   pub recipient: AgentPubKey,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryStateInput {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
}