mod inmail;
mod distribution;
mod pending_mail;
mod pending_ack;
mod outack;
mod manifest_confirmation;
mod delivery_confirmation;

use hdk::prelude::*;


/// Possible states of an InMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InMailState {
    /// InMail committed, no OutAck
    Unacknowledged,
    /// OutAck committed, no confirmation, no pending
    AckUnsent,
    /// OutAck committed, PendingAck available
    AckPending,
    /// OutAck committed, confirmation commited
    AckDelivered,
    /// Delete entry commited
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
pub struct Manifest {
    parcel_type: String,
    total_parcel_size: u64,
    payload: Vec<EntryHash>,
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

