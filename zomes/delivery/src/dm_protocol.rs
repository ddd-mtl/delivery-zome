use hdk::prelude::*;

use crate::{
   file::{FileChunk, FileManifest},
};
use crate::entries::{Mail, ParcelDescription};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryProtocol {
    Failure(String),
    Success(String),
    ReceptionRequest(ReceptionRequestMessage),
    Ack(AckMessage),
    Chunk(FileChunk),
    FileManifest(FileManifest),
    RequestChunk(EntryHash),
    RequestManifest(EntryHash),
    UnknownEntry,
    Ping,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct ReceptionRequestMessage {
    pub description: ParcelDescription,
    pub sender_description_signature: Signature,
    pub sender_distribution_eh: EntryHash,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct AckMessage {
    pub outmail_eh: EntryHash,
    pub ack_signature: Signature,
}
