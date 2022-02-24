use hdk::prelude::*;

use crate::parcel::*;


/// Entry representing a received Manifest
#[hdk_entry(id = "DeliveryNotice", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
   pub distribution_eh: EntryHash,
   pub sender: AgentPubKey,
   pub sender_summary_signature: Signature,
   pub parcel_summary: ParcelSummary,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryReceipt", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryReceipt {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
   pub recipient_signature: Signature,
   //pub date_of_reception: u64,
}


// pub enum ReceptionResponse {
//     Accepted((HeaderHash, Signature)),
//     Refused,
// }

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryReply", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryReply {
   pub notice_eh: EntryHash,
   pub has_accepted: bool,
}


#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DistributionStrategy {
   /// DM first, DHT otherwise
   NORMAL,
   /// Publish to DHT unencrypted,
   PUBLIC,
   /// Encrypt to recipients on DHT
   DHT_ONLY,
   /// Only via DM
   DM_ONLY,
}

/// Entry representing a request to send a Parcel to one or multiple recipients
#[hdk_entry(id = "Distribution", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Distribution {
   pub recipients: Vec<AgentPubKey>,
   pub parcel_summary: ParcelSummary,
   pub strategy: DistributionStrategy,
   pub summary_signature: Signature,
   //pub can_share_between_recipients: bool, // Make recipient list "public" to recipients
}


/// Entry for confirming a manifest has been well received by a recipient
#[hdk_entry(id = "NoticeReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct NoticeReceived {
   pub distribution_eh: EntryHash,
   pub recipient: AgentPubKey,
   pub recipient_manifest_signature: Signature,
   pub date_of_reception: u64,
}


/// Entry representing a file chunk.
#[hdk_entry(id = "ParcelChunk", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
   pub data: String,
}


#[hdk_entry(id = "ParcelManifest", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelManifest {
   pub name: String,
   pub entry_id: String,
   //pub data_hash: String,
   pub size: usize,
   pub chunks: Vec<EntryHash>,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry(id = "ParcelReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedHeaderHashed, // signed header of parcel's Element
}


/// List of structs that PendingItem can embed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemKind {
   DeliveryNotice,
   DeliveryReply,
   ParcelReceived,
   Entry,
   // ParcelManifest
   ParcelChunk,
}

/// A Public Entry representing an encrypted private Entry on the DHT
/// waiting to be received by some recipient.
/// The recipient is the agentId where the entry is linked from.
/// The Entry is encrypted with the recipient's public encryption key.
#[hdk_entry(id = "PendingItem")]
#[derive(Clone, PartialEq)]
pub struct PendingItem {
   pub kind: ItemKind,
   //pub app_type: &'static str,
   pub author_signature: Signature, // Signature of the Entry's author
   pub encrypted_data: XSalsa20Poly1305EncryptedData,
   pub distribution_eh: EntryHash,
}


/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "ReplyReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ReplyReceived {
   pub distribution_eh: EntryHash,
   //pub date_of_reply: u64,
   pub recipient: AgentPubKey,
   pub has_accepted: bool,
   pub recipient_signature: Signature,
}


/// Entry representing the Public Encryption Key of an Agent
#[hdk_entry(id = "PubEncKey", visibility = "public")]
#[derive(Clone, PartialEq)]
pub struct PubEncKey {
   pub value: X25519PubKey,
}

impl PubEncKey {
   pub fn new() -> Self {
      let value = create_x25519_keypair()
         .expect("Create Keypair should work");
      Self {
         value,
      }
   }
}
