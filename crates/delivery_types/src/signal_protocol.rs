use hdi::prelude::*;
use crate::*;


#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct SystemSignal {
    pub System: SystemSignalProtocol,
}


#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct DeliverySignal {
    pub from: AgentPubKey,
    pub pulses: Vec<DeliverySignalProtocol>,
}


/// Protocol for notifying the ViewModel (UI) of system level events
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemSignalProtocol {
    PostCommitStart {entry_type: String},
    PostCommitEnd {entry_type: String, succeeded: bool},
    SelfCallStart {zome_name: String, fn_name: String},
    SelfCallEnd {zome_name: String, fn_name: String, succeeded: bool},
}


/// Protocol for notifying the ViewModel (UI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeliverySignalProtocol {
    System(SystemSignalProtocol),
    Gossip(DeliveryGossipProtocol),
    Entry((EntryInfo, DeliveryEntryKind)),
    //ReceivedChunk((Vec<EntryHash>, usize)), // EntryHash of DeliveryNotice for the Chunk
}


#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum EntryStateChange {
    None,
    Created,
    Deleted,
    Updated,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct EntryInfo {
    pub hash: AnyDhtHash,
    pub ts: Timestamp,
    pub author: AgentPubKey,
    pub state: EntryStateChange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeliveryEntryKind {
    DeliveryNotice(DeliveryNotice),
    ReceptionAck(ReceptionAck),
    NoticeReply(NoticeReply),
    Distribution(Distribution),
    ParcelManifest(ParcelManifest),
    ParcelChunk(ParcelChunk),
    NoticeAck(NoticeAck),
    ReplyAck(ReplyAck),
    ReceptionProof(ReceptionProof),
    PendingItem(PendingItem),
    PublicParcel(ParcelReference),
}


// ///---------------------------------------------------------------------------------------
// /// For sweettest
// ///---------------------------------------------------------------------------------------
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum SignalKind {
//     NewNotice,
//     NewReplyAck,
//     NewReceptionProof,
//     NewReceptionAck,
// }
//
//
// impl SignalProtocol {
//     pub fn is(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
//         match kind {
//             SignalKind::NewNotice => {
//                 if let SignalProtocol::NewNotice((_notice_eh, notice, _ts)) = self {
//                     return &notice.distribution_ah == eh;
//                 }
//                 false
//             },
//             SignalKind::NewReplyAck => {
//                 if let SignalProtocol::NewReplyAck((_eh, entry)) = self {
//                     return &entry.distribution_ah == eh;
//                 }
//                 false
//             },
//             SignalKind::NewReceptionProof => {
//                 if let SignalProtocol::NewReceptionProof((_eh, received)) = self {
//                     return &received.parcel_eh == eh;
//                 }
//                 false
//             },
//             SignalKind::NewReceptionAck => {
//                 if let SignalProtocol::NewReceptionAck((_eh, entry)) = self {
//                     return &entry.distribution_ah == eh;
//                 }
//                 false
//             },
//         }
//     }
// }
