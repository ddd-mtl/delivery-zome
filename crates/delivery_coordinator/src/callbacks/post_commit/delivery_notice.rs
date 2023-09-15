use hdk::prelude::*;
use zome_utils::{call_self/*, decode_response*/};

use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_DeliveryNotice(sah: &SignedActionHashed, entry: Entry, notice_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_DeliveryNotice() {:?}", notice_eh);
    let me = agent_info()?.agent_latest_pubkey;
    let notice = DeliveryNotice::try_from(entry)?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::NewNotice((notice_eh.to_owned(), notice.clone(), sah.hashed.content.timestamp())));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    }
    /// Create NoticeAck and pack it
    let signature = sign(me.clone(), notice.summary.clone())?;
    let ack: NoticeAck = NoticeAck {
        distribution_ah: notice.distribution_ah.clone(),
        recipient: me,
        recipient_summary_signature: signature,
        //date_of_reception: sys_time()?,
    };
    let pending_item = pack_notice_ack(ack, notice.sender.clone())?;
    /// Send NoticeAck to sender
    let res = send_item(
        notice.sender.clone(),
        pending_item,
        notice.summary.distribution_strategy.clone(),
    );
    if let Err(e) = res {
        warn!("send_item() during DeliveryNotice::post_commit() failed: {}", e);
    }
    /// Check for duplicate Parcel
    let has_parcel = has_entry(notice.summary.parcel_reference.eh)?;
    if has_parcel {
        /// Automatically reject notice
        debug!("Already have this parcel");
        let input = RespondToNoticeInput {
            notice_eh: notice_eh.clone(),
            has_accepted: false,
        };
        let _response = call_self("respond_to_notice", input)?;
        //let _reply_eh: EntryHash = decode_response(response)?;
        return Ok(());
    }
    /// Done
    Ok(())
}
