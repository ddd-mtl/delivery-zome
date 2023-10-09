use std::fmt;
use hdk::prelude::*;
//use zome_delivery_integrity::*;
use zome_delivery_types::*;


///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectMessage {
    pub from: AgentPubKey,
    pub msg: DeliveryProtocol,
}


///
pub fn failure(reason: &str) -> DeliveryProtocol {
    warn!(reason);
    return DeliveryProtocol::Failure(reason.to_string());
}


///
pub fn failure_err(reason: &str, err: WasmError) -> DeliveryProtocol {
    let msg = format!("{}: {:?}", reason, err);
    warn!("{}", msg);
    return DeliveryProtocol::Failure(msg);
}


///  Protocol for sending data between agents
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryProtocol {
    Failure(String),
    Success(Signature),
    Item(PendingItem),
    ParcelRequest(ActionHash), // distribution_ah
    ParcelResponse(Entry),
    ChunkRequest(EntryHash),
    ChunkResponse(ParcelChunk),
    PublicParcelPublished((Timestamp, ParcelReference, AgentPubKey)),
    //UnknownEntry, // TODO implement
    Ping,
    Pong,
}

impl fmt::Display for DeliveryProtocol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str: String = match self {
            DeliveryProtocol::Failure(str) => format!("Failure: {}", str),
            DeliveryProtocol::Success(_) => "Success".to_owned(),
            DeliveryProtocol::Item(item) => format!("Item: {:?}", item.kind),
            DeliveryProtocol::ParcelRequest(eh) => format!("ParcelRequest: {}", eh),
            DeliveryProtocol::ParcelResponse(_entry) => format!("ParcelResponse"),
            DeliveryProtocol::ChunkRequest(eh) => format!("ChunkRequest: {}", eh),
            DeliveryProtocol::ChunkResponse(_chunk) => format!("ChunkResponse"),
            DeliveryProtocol::PublicParcelPublished((_ts, _pr, author)) => format!("NewPublicParcel from {}", author),
            // DeliveryProtocol::UnknownEntry => "UnknownEntry".to_owned(),
            DeliveryProtocol::Ping => "Ping".to_owned(),
            DeliveryProtocol::Pong => "Pong".to_owned(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}

// impl From<ExternResult<()>> for DeliveryProtocol {
//     fn from(result: ExternResult<()>) -> Self {
//         match result {
//             Err(err) => failure_err("", err),
//             Ok(_) => DeliveryProtocol::Success,
//         }
//     }
// }
