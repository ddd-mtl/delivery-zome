use hdk::prelude::*;
use zome_utils::{call_self/*, decode_response*/};

use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_create_DeliveryNotice(_sah: &SignedActionHashed, create: &Create, entry: Entry) -> ExternResult<DeliveryEntryKind> {
    debug!("post_commit_DeliveryNotice() {:?}", create.entry_hash);
    let me = agent_info()?.agent_latest_pubkey;
    let notice = DeliveryNotice::try_from(entry)?;
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
    let has_parcel = has_entry(notice.summary.parcel_reference.parcel_eh.clone())?;
    if has_parcel {
        /// Automatically reject notice
        debug!("Already have this parcel");
        let input = RespondToNoticeInput {
            notice_eh: create.entry_hash.clone(),
            has_accepted: false,
        };
        let _response = call_self("respond_to_notice", input)?;
        //let _reply_eh: EntryHash = decode_response(response)?;
    }
    /// Done
    Ok(DeliveryEntryKind::DeliveryNotice(notice))
}
