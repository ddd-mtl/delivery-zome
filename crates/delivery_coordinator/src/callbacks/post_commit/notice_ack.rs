use hdk::prelude::*;
use zome_delivery_types::*;
use crate::emit_self_signal;


///
pub fn post_commit_NoticeAck(sah: &SignedActionHashed,  entry: Entry, eh: &EntryHash) -> ExternResult<()> {
    let ack = NoticeAck::try_from(entry)?;
    /// Emit Signal
    let res = emit_self_signal(DeliverySignalProtocol::NewNoticeAck((eh.to_owned(), sah.hashed.content.timestamp(), ack)));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    }
    /// Done
    Ok(())
}
