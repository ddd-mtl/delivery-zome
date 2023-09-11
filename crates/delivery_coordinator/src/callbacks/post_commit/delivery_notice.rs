use hdk::prelude::*;

use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_DeliveryNotice(sah: &SignedActionHashed, entry: Entry, eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_DeliveryNotice() {:?}", eh);
    let me = agent_info()?.agent_latest_pubkey;
    let notice = DeliveryNotice::try_from(entry)?;
    /// Create NoticeReceived and pack it
    let signature = sign(me.clone(), notice.summary.clone())?;
    let ack: NoticeReceived = NoticeReceived {
        distribution_eh: notice.distribution_eh.clone(),
        recipient: me,
        recipient_summary_signature: signature,
        //date_of_reception: sys_time()?,
    };
    let pending_item = pack_notice_received(ack, notice.sender.clone())?;
    /// Send NoticeReceived to sender
    let res = send_item(
        notice.sender.clone(),
        pending_item,
        notice.summary.distribution_strategy.clone(),
    );
    if let Err(e) = res {
        warn!("send_item() during DeliveryNotice::post_commit() failed: {}", e);
    }
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::ReceivedNotice((eh.to_owned(), sah.hashed.content.timestamp(), notice)));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    } else {
        debug!("Emit signal successful!");
    }
    /// Done
    Ok(())
}
