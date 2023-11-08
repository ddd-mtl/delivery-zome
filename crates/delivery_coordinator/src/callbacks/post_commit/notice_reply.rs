use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::*;



/// Once committed, send reply to sender
pub fn post_commit_NoticeReply(sah: &SignedActionHashed, entry: Entry, reply_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_NoticeReply() {:?}", reply_eh);
    let reply = NoticeReply::try_from(entry)?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::NewReply((reply_eh.to_owned(), sah.hashed.content.timestamp(), reply.clone())));
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    /// Send reply to sender
    let _ = send_reply(reply)?;
    Ok(())
}


///
fn send_reply(delivery_reply: NoticeReply) -> ExternResult<()> {
    /// Get DeliveryNotice
    let notice: DeliveryNotice = get_typed_from_eh(delivery_reply.notice_eh.clone())?;
    /// Create PendingItem from NoticeReply
    let pending_item = pack_reply(delivery_reply.clone(), notice.distribution_ah.clone(), notice.sender.clone())?;
    /// Send it to sender
    let res = send_item(
        notice.sender,
        pending_item,
        notice.summary.distribution_strategy,
    );
    if let Err(e) = res {
        warn!("send_item() during NoticeReply::post_commit() failed: {}", e);
    } else {
        /// Fetch parcel if it has been accepted by this agent (recipient)
        if let SendSuccessKind::OK_DIRECT(_signature) = res.unwrap() {
            // FIXME
            // let valid = verify_signature(recipient.clone(), signature.clone(), pending_item.clone())?;
            // if !valid {
            //     warn!("Sender failed to sign NoticeReply. Suspicious behavior.");
            //     return zome_error!("Sender failed to sign NoticeReply. Suspicious behavior.");
            // }
            if delivery_reply.has_accepted {
                let response = call_self("fetch_parcel", delivery_reply.notice_eh.clone())?;
                debug!("fetch_parcel() response: {:?}", response);
                //assert!(matches!(response, ZomeCallResponse::Ok { .. }));
            }
        }
    }
    /// Done
    Ok(())
}