use hdk::prelude::*;
use zome_delivery_types::*;
//use crate::*;
//use zome_utils::*;


///
pub fn post_commit_create_ReplyAck(_sah: &SignedActionHashed, create: &Create, entry: Entry) -> ExternResult<DeliveryEntryKind> {
   debug!("post_commit_create_ReplyAck() {:?}", create.entry_hash);
   let reply_ack = ReplyAck::try_from(entry)?;
   /// Check signature
   // FIXME
   //let valid = verify_signature(reply.recipient, reply.recipient_signature, )?;
   /// Done
   Ok(DeliveryEntryKind::ReplyAck(reply_ack))
}
