use hdk::prelude::*;

use zome_delivery_types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice(DeliveryNotice),
    ReceivedReply(ReplyReceived),
    ReceivedParcel(ParcelReceived),
    ReceivedReceipt(DeliveryReceipt),
}
