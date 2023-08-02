use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::*;



///
pub fn post_commit_DeliveryReply(entry: Entry, reply_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_DeliveryReply() {:?}", reply_eh);
    let delivery_reply = DeliveryReply::try_from(entry)?;
    /// Get DeliveryNotice
    let notice: DeliveryNotice = get_typed_from_eh(delivery_reply.notice_eh.clone())?;
    /// Create PendingItem from DeliveryReply
    let pending_item = pack_reply(delivery_reply.clone(), notice.distribution_eh.clone(), notice.sender.clone())?;
    /// Send item to recipient
    let res = send_item(
        notice.sender,
        pending_item,
        notice.summary.distribution_strategy,
    );
    if let Err(e) = res {
        warn!("send_item() during DeliveryReply::post_commit() failed: {}", e);
    }
    /// Try to retrieve parcel if it has been accepted
    if delivery_reply.has_accepted {
        let response = call_self("fetch_parcel", delivery_reply.notice_eh.clone())?;
        debug!("fetch_parcel() response: {:?}", response);
        assert!(matches!(response, ZomeCallResponse::Ok { .. }));
    }
    /// Done
    Ok(())
}