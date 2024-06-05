//! All Zome function input types

use hdi::prelude::*;

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeParcelInput {
   pub recipients: Vec<AgentPubKey>,
   pub strategy: DistributionStrategy,
   pub parcel_reference: ParcelReference,
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
   pub state: (NoticeState, Vec<EntryHash>),
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeliveryNoticeQueryField {
   Sender(AgentPubKey),
   Distribution(ActionHash),
   Parcel(EntryHash)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceptionProofQueryField {
   Notice(EntryHash),
   Parcel(EntryHash)
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoticeAckQueryField {
   Recipient(AgentPubKey),
   Distribution(ActionHash)
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitPendingItemInput {
   pub item: PendingItem,
   pub recipient: AgentPubKey,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDeliveryStateInput {
   pub distribution_ah: ActionHash,
   pub recipient: AgentPubKey,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BroadcastInput {
   pub peers: Vec<AgentPubKey>,
   pub pr: ParcelReference,
   pub timestamp: Timestamp,
   pub removed: bool,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicParcelRecord {
   pub pr_eh: EntryHash,
   pub pp_eh: EntryHash,
   pub description: ParcelDescription,
   pub creation_ts: Timestamp,
   pub author: AgentPubKey,
   pub deleteInfo: Option<(Timestamp, AgentPubKey)>,
}
