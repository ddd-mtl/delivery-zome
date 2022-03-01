use hdk::prelude::*;

use crate::parcel::*;

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
pub enum DeliveryNoticeQueryField {
   Sender(AgentPubKey),
   Parcel(EntryHash)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParcelReceivedQueryField {
   Notice(EntryHash),
   Parcel(EntryHash)
}
