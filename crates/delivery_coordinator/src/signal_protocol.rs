use hdk::prelude::*;
use zome_delivery_types::*;


/// Protocol for notifying the ViewModel (UI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    NewChunk((EntryHash, usize)), // ParcelManifest EntryHash
    NewManifest(ParcelManifest),
    NewDistribution((EntryHash, Timestamp, Distribution)),
    NewNotice((EntryHash, Timestamp, DeliveryNotice)),
    NewNoticeAck(NoticeAck),
    NewReply(NoticeReply),
    NewReplyAck(ReplyAck),
    NewReceptionProof(ReceptionProof),
    NewReceptionAck(ReceptionAck),
    NewPendingItem(PendingItem),
}


///---------------------------------------------------------------------------------------
/// For sweettest
///---------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalKind {
    NewNotice,
    NewReplyAck,
    NewReceptionProof,
    NewReceptionAck,
}


impl SignalProtocol {
    pub fn is(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
        match kind {
            SignalKind::NewNotice => {
                if let SignalProtocol::NewNotice((_notice_eh, _ts, notice)) = self {
                    return &notice.distribution_eh == eh;
                }
                false
            },
            SignalKind::NewReplyAck => {
                if let SignalProtocol::NewReplyAck(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
            SignalKind::NewReceptionProof => {
                if let SignalProtocol::NewReceptionProof(received) = self {
                    return &received.parcel_eh == eh;
                }
                false
            },
            SignalKind::NewReceptionAck => {
                if let SignalProtocol::NewReceptionAck(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
        }
    }
}