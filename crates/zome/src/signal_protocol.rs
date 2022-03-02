use hdk::prelude::*;

use zome_delivery_types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice(DeliveryNotice),
    ReceivedReply(ReplyReceived),
    //ReceivedParcel(EntryHash),
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct ReceivedMail {
//     pub item: MailItem,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct ReceivedAck {
//     pub from: AgentPubKey,
//     pub for_mail: HeaderHash,
// }
