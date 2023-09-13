use hdk::prelude::*;
use zome_delivery_types::*;


/// Protocol for notifying the ViewModel (UI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    NewManifest((EntryHash, ParcelManifest)),
    ReceivedChunk((EntryHash, usize)), // EntryHash of DeliveryNotice for the Chunk
    NewDistribution((ActionHash, Distribution, Timestamp)),
    NewNotice((EntryHash, DeliveryNotice, Timestamp)),
    NewNoticeAck((EntryHash, NoticeAck)),
    NewReply((EntryHash, NoticeReply)),
    NewReplyAck((EntryHash, ReplyAck)),
    NewReceptionProof((EntryHash, ReceptionProof)),
    NewReceptionAck((EntryHash, ReceptionAck)),
    NewPendingItem((EntryHash, PendingItem)),
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