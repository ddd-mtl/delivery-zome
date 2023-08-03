//! All zome entry types

use hdi::prelude::*;

use crate::*;


/// Entry representing a received Manifest
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
   pub distribution_eh: EntryHash,
   pub summary: DeliverySummary,
   pub sender: AgentPubKey,
   pub sender_summary_signature: Signature,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeliveryReceipt {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
   pub recipient_signature: Signature,
   //pub date_of_reception: u64,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeliveryReply {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}

/// Entry representing a request to send a Parcel to one or multiple recipients
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Distribution {
   pub recipients: Vec<AgentPubKey>,
   pub delivery_summary: DeliverySummary,
   pub summary_signature: Signature,
}


/// Entry for confirming a manifest has been well received by a recipient
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct NoticeReceived {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
   pub recipient_summary_signature: Signature,
   pub date_of_reception: Timestamp,
}


/// Entry representing a file chunk.
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
   pub data: String,
}

/// WARN : Change MANIFEST_ENTRY_NAME const when renaming
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParcelManifest {
   pub name: String,
   pub custum_entry_type: String,
   //pub data_hash: String,
   pub size: usize,
   pub chunks: Vec<EntryHash>,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedActionHashed, // signed Action of parcel's record
}


/// List of structs that PendingItem can embed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemKind {
   /// Sent by recipient
   NoticeReceived,
   DeliveryReply,
   ParcelReceived,
   /// Sent by sender
   DeliveryNotice,
   AppEntryBytes,
   ParcelChunk,
   // ParcelManifest
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
   pub distribution_eh: EntryHash,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ReplyReceived {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
   pub has_accepted: bool,
   pub recipient_signature: Signature,
   //pub date_of_reply: u64,
}
