use hdi::prelude::*;
use crate::*;

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
pub enum SignalProtocol {
    System(SystemSignalProtocol),
    NewLocalManifest((EntryHash, Timestamp, ParcelManifest)),
    NewLocalChunk((EntryHash, ParcelChunk)),
    ReceivedChunk((Vec<EntryHash>, usize)), // EntryHash of DeliveryNotice for the Chunk
    NewDistribution((ActionHash, Timestamp, Distribution)),
    NewNotice((EntryHash, Timestamp, DeliveryNotice)),
    NewNoticeAck((EntryHash, Timestamp, NoticeAck)),
    NewReply((EntryHash, Timestamp, NoticeReply)),
    NewReplyAck((EntryHash, Timestamp, ReplyAck)),
    NewReceptionProof((EntryHash, Timestamp, ReceptionProof)),
    NewReceptionAck((EntryHash, Timestamp, ReceptionAck)),
    NewPendingItem((EntryHash, PendingItem)),
    NewPublicParcel((EntryHash, Timestamp, ParcelReference, AgentPubKey)),
    RemovedPublicParcel((EntryHash, Timestamp, ParcelReference, AgentPubKey)),
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
