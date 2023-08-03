use hdk::prelude::*;

use zome_delivery_types::*;
//use zome_delivery_integrity::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalKind {
    ReceivedNotice,
    ReceivedReply,
    ReceivedParcel,
    ReceivedReceipt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice((EntryHash, DeliveryNotice)),
    ReceivedAck(NoticeReceived),
    ReceivedReply(ReplyReceived),
    ReceivedParcel(ParcelReceived),
    ReceivedReceipt(DeliveryReceipt),
}

impl SignalProtocol {
    pub fn is(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
        match kind {
            SignalKind::ReceivedNotice => {
                if let SignalProtocol::ReceivedNotice((_notice_eh, notice)) = self {
                    return &notice.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReply => {
                if let SignalProtocol::ReceivedReply(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedParcel => {
                if let SignalProtocol::ReceivedParcel(received) = self {
                    return &received.parcel_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReceipt => {
                if let SignalProtocol::ReceivedReceipt(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
        }
    }
}