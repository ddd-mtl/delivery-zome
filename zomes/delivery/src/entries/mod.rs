pub mod delivery_notice;
pub mod delivery_receipt;
pub mod delivery_reply;
pub mod distribution;
pub mod notice_received;
pub mod parcel_chunk;
pub mod parcel_manifest;
pub mod parcel_received;
pub mod pending_item;
pub mod pub_enc_key;
pub mod reply_received;


pub use delivery_notice::*;
pub use delivery_receipt::*;
pub use delivery_reply::*;
pub use distribution::*;
pub use notice_received::*;
pub use parcel_chunk::*;
pub use parcel_manifest::*;
pub use parcel_received::*;
pub use pending_item::*;
pub use pub_enc_key::*;
pub use reply_received::*;


use hdk::prelude::*;

/// Possible states of an InMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum IncomingDeliveryState {
    /// DeliveryNotification committed
    ManifestReceived,
    /// ReceptionConfirmation(yes) committed
    Accepted,
    /// ReceptionConfirmation(no) committed
    Refused,
    /// Parcel committed
    Received,
    /// Parcel deleted ???
    Deleted,
}

/// State of a single delivery of a mail or ack to a unique recipient
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeliveryState {
    /// Initial state ; Distribution Entry committed
    Unsent,
    /// There is a link to a PendingItem entry holding the Manifest and inbox link is alive
    PendingManifest,
    /// ManifestConfirmation committed,
    ManifestDelivered,
    /// There is a link to a PendingItem entry holding the Parcel and inbox link is alive
    PendingParcel,
    /// DeliveryConfirmation committed, We have proof object has been received:
    /// DM has been sent successfully or link to pending has been deleted
    ParcelDelivered,
}


/// Possible states of an OutMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DistributionState {
    /// Initial state ; Distribution committed
    Unsent,
    /// (yellow) All deliveries have been sent (no Unsent state)
    AllSent,
    /// (black) All deliveries have been received (no Unsent or PendingManifest state)
    AllManifestReceived,
    /// (green) Has a DeliveryConfirmation for each recipient
    AllParcelsReceived,
    /// (red) Delete entry commited
    Deleted,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeliveryItem {
    pub hh: HeaderHash,
    pub author: AgentPubKey,
    pub parcel: Parcel,
    pub state: DistributionState,
    // pub delivery_states: Map<AgentPubKey, DeliveryState>
    pub recipients: Vec<AgentPubKey>,
    pub send_date: i64,
}

