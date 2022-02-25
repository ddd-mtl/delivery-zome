use hdk::prelude::*;
use std::fmt;
use zome_delivery_types::*;


pub fn failure(reason: &str) -> DeliveryProtocol {
    warn!(reason);
    return DeliveryProtocol::Failure(reason.to_string());
}

pub fn failure_err(reason: &str, err: WasmError) -> DeliveryProtocol {
    warn!("{}: {:?}", reason, err);
    return DeliveryProtocol::Failure(reason.to_string());
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryProtocol {
    Failure(String),
    Success,
    Item(PendingItem),
    /// Distribution EntryHash of the Parcel
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
            DeliveryProtocol::Success => "Success".to_owned(),
            DeliveryProtocol::Item(item) => format!("Item: {:?}", item.kind),
            DeliveryProtocol::ParcelRequest(eh) => format!("ParcelRequest: {}", eh),
            DeliveryProtocol::ParcelResponse(_entry) => format!("ParcelResponse"),
            DeliveryProtocol::ChunkRequest(eh) => format!("ChunkRequest: {}", eh),
            DeliveryProtocol::ChunkResponse(_chunk) => format!("ChunkResponse"),
            DeliveryProtocol::UnknownEntry => "UnknownEntry".to_owned(),
            DeliveryProtocol::Ping => "Ping".to_owned(),
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
