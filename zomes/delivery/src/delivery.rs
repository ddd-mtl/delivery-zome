use hdk::prelude::*;

use crate::states::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OutgoingDeliveryItem {
   pub state: DeliveryState,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IncomingDeliveryItem {
   pub hh: HeaderHash,
   pub author: AgentPubKey,
   pub parcel: Parcel,
   pub state: NoticeState,
   // pub delivery_states: Map<AgentPubKey, DeliveryState>
   pub recipients: Vec<AgentPubKey>,
   pub send_date: i64,
}
