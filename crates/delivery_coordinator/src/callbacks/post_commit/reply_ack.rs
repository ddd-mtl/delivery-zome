use hdk::prelude::*;
use zome_delivery_types::*;
//use crate::*;
//use zome_utils::*;


///
pub fn post_commit_create_ReplyAck(_sah: &SignedActionHashed, eh: &EntryHash, entry: Entry) -> ExternResult<()> {
   debug!("post_commit_create_ReplyAck() {:?}", eh);
   let _reply_ack = ReplyAck::try_from(entry)?;
   /// Check signature
   // TODO
   //let valid = verify_signature(reply.recipient, reply.recipient_signature, )?;
   /// Done
   Ok(())
}
