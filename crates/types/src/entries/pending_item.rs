use hdk::prelude::*;

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
