use hdk::prelude::*;

use crate::entries::*;
use crate::parcel::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeParcelInput {
   pub recipients: Vec<AgentPubKey>,
   pub strategy: DistributionStrategy,
   pub parcel_kind: ParcelKind,
   pub parcel_eh: EntryHash,
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RespondToNoticeInput {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}
