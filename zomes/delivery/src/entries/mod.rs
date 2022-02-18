pub mod delivery_notification;
pub mod distribution;
pub mod pending_item;
pub mod description_confirmation;
pub mod delivery_confirmation;
pub mod parcel_chunk;
pub mod pub_enc_key;
pub mod reception_confirmation;
pub mod parcel_manifest;

pub use delivery_notification::*;
pub use distribution::*;
pub use pending_item::*;
pub use description_confirmation::*;
pub use delivery_confirmation::*;
pub use parcel_chunk::*;
pub use pub_enc_key::*;
pub use reception_confirmation::*;
pub use parcel_manifest::*;


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

