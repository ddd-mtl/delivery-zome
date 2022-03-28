use hdk::prelude::*;

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice(DeliveryNotice),
    ReceivedReply(ReplyReceived),
    ReceivedParcel(ParcelReceived),
    ReceivedReceipt(DeliveryReceipt),
}
