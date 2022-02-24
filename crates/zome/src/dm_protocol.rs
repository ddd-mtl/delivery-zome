use hdk::prelude::*;
use std::fmt;
use zome_delivery_types::*;


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

impl fmt::Display for DeliveryProtocol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str: String = match self {
            DeliveryProtocol::Failure(str) => format!("Failure: {}", str),
            DeliveryProtocol::Success(str) => format!("Success: {}", str),
            DeliveryProtocol::Item(item) => format!("Item: {:?}", item.kind),
            DeliveryProtocol::ParcelRequest(eh) => format!("ParcelRequest: {}", eh),
            DeliveryProtocol::ParcelResponse(_entry) => format!("ParcelResponse"),
            DeliveryProtocol::ChunkRequest(eh) => format!("ChunkRequest: {}", eh),
            DeliveryProtocol::ChunkResponse(_chunk) => format!("ChunkResponse"),
            DeliveryProtocol::UnknownEntry => "UnknownEntry".to_owned(),
            DeliveryProtocol::Ping=> "Ping".to_owned(),
            DeliveryProtocol::Pong => "Pong".to_owned(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
// pub struct AckMessage {
//     pub outmail_eh: EntryHash,
//     pub ack_signature: Signature,
// }
