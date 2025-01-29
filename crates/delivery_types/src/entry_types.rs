//! All zome entry types

use hdi::prelude::*;

use crate::*;


/// Entry representing a request to send a Parcel to one or multiple recipients
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Distribution {
   pub recipients: Vec<ActionHash>,
   pub delivery_summary: DeliverySummary,
   pub summary_signature: Signature, // signed by entry author
}


/// Entry representing a received delivery request
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
   pub distribution_ah: ActionHash,
   pub summary: DeliverySummary,
   pub sender: AgentPubKey,
   pub sender_summary_signature: Signature,
}


/// Entry for confirming a request has been well received by a recipient
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct NoticeAck {
   pub distribution_ah: ActionHash,
   pub recipient: ActionHash,
   pub signing_recipient: AgentPubKey,
   pub recipient_summary_signature: Signature,
}


/// Entry for accepting or refusing a delivery
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct NoticeReply {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}


/// Entry for confirming a recipient's reply on the sender's side
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ReplyAck {
   pub distribution_ah: ActionHash,
   pub recipient: ActionHash,
   pub has_accepted: bool,
   pub signing_recipient: AgentPubKey,
   pub recipient_signature: Signature,
   //pub date_of_reply: u64,
}


/// Entry representing a chunk a data (for a parcel)
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
   pub data_hash: String,
   pub data: String,
}


/// Entry for holding arbitrary data for a Parcel.
/// Used as a universel way to send data.
/// WARN: Change MANIFEST_ENTRY_NAME const when renaming
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParcelManifest {
   pub description: ParcelDescription,
   pub data_hash: String,
   pub chunks: Vec<EntryHash>,
}


/// Entry for confirming a delivery has been well received or refused by the recipient.
/// TODO: This should be a private link instead of an entry
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ReceptionProof {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedActionHashed, // signed Action of parcel's record
}


/// Entry for confirming a delivery has been well received or refused by the recipient.
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ReceptionAck {
   pub distribution_ah: ActionHash,
   pub recipient: ActionHash,
   pub signing_recipient: AgentPubKey,
   pub recipient_signature: Signature,
   //pub date_of_reception: u64,
}


/// A Public Entry representing an encrypted private Entry on the DHT
/// waiting to be received by some recipient.
/// The Entry is encrypted with the recipient's public encryption key.
/// The recipient is the agentId where the entry is linked from.
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct PendingItem {
   pub kind: ItemKind,
   pub author: AgentPubKey,
   pub author_signature: Signature, // Signature of the Entry's author
   pub encrypted_data: XSalsa20Poly1305EncryptedData,
   pub distribution_ah: ActionHash,
}


/// List of structs that PendingItem can embed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemKind {
   /// Sent by recipient
   NoticeAck,
   NoticeReply,
   ReceptionProof,
   /// Sent by sender
   DeliveryNotice,
   ParcelChunk,
   AppEntryBytes, // ParcelManifest is sent as AppEntryBytes
}

impl ItemKind {
   pub fn can_link_to_distribution(&self) -> bool {
      match self {
         Self::DeliveryNotice => true,
         Self::AppEntryBytes => true,
         Self::ParcelChunk => true,
         _ => false,

      }
   }
}



