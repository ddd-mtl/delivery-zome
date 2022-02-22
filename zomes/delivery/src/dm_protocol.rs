use hdk::prelude::*;

use crate::entries::*;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryProtocol {
    Failure(String),
    Success(String),
    Item(PendingItem),
    ParcelRequest(EntryHash),
    ParcelResponse(Entry),
    ChunkRequest(EntryHash),
    ChunkResponse(ParcelChunk),
    UnknownEntry,
    Ping,
    Pong,
}


// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
// pub struct AckMessage {
//     pub outmail_eh: EntryHash,
//     pub ack_signature: Signature,
// }
