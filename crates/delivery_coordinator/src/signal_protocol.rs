use hdk::prelude::*;

use zome_delivery_types::*;
//use zome_delivery_integrity::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalKind {
    ReceivedNotice,
    ReceivedReplyAck,
    ReceivedReceptionProof,
    ReceivedReceptionAck,
}

///  Protocol for sending data to the agent's UI
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice((EntryHash, Timestamp, DeliveryNotice)),
    ReceivedNoticeAck(NoticeAck),
    ReceivedReplyAck(ReplyAck),
    CreatedReceptionProof(ReceptionProof),
    ReceivedReceptionAck(ReceptionAck),
    CreatedDistribution((EntryHash, Timestamp, Distribution)),
}

/// For sweettest?
impl SignalProtocol {
    pub fn is(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
        match kind {
            SignalKind::ReceivedNotice => {
                if let SignalProtocol::ReceivedNotice((_notice_eh, _ts, notice)) = self {
                    return &notice.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReplyAck => {
                if let SignalProtocol::ReceivedReplyAck(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReceptionProof => {
                if let SignalProtocol::CreatedReceptionProof(received) = self {
                    return &received.parcel_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReceptionAck => {
                if let SignalProtocol::ReceivedReceptionAck(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
        }
    }
}