use hdk::prelude::*;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedMail(MailItem),
    ReceivedAck(ReceivedAck),
    ReceivedFile(FileManifest),
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct ReceivedMail {
//     pub item: MailItem,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReceivedAck {
    pub from: AgentPubKey,
    pub for_mail: HeaderHash,
}
