use hdk::prelude::*;

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "ReplyReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ReplyReceived {
   pub distribution_eh: EntryHash,
   //pub date_of_reply: u64,
   pub recipient: AgentPubKey,
   pub has_accepted: bool,
   pub recipient_signature: Signature,
}