use hdk::prelude::*;

use crate::{entries::*, get_typed_from_eh, send_item, utils::*};
//use crate::functions::fetch_parcel;


// pub enum ReceptionResponse {
//     Accepted((HeaderHash, Signature)),
//     Refused,
// }

/// Entry for confirming a delivery has been well received or refused by a recipient
#[hdk_entry(id = "DeliveryReply", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryReply {
    pub notice_eh: EntryHash,
    pub has_accepted: bool,
}

impl DeliveryReply {
    ///
    pub fn post_commit(reply_eh: &EntryHash, reply: Self) -> ExternResult<()> {
        debug!("post_commit_DeliveryReply() {:?}", reply_eh);
        /// Get DeliveryNotice
        let notice: DeliveryNotice = get_typed_from_eh(reply.notice_eh.clone())?;
        /// Create PendingItem from DeliveryReply
        let pending_item = PendingItem::from_reply(
            reply.clone(),
            notice.sender.clone(),
        )?;
        /// Sign DeliveryReply
        let me = agent_info()?.agent_latest_pubkey;
        let signature = sign(me.clone(), reply.clone())?;
        /// Send item to recipient
        let _res = send_item(
            notice.sender,
            notice.distribution_eh.clone(),
            pending_item,
            signature,
        );
        /// Try to retrieve parcel if it has been accepted
        if reply.has_accepted {
            let response = call_self("fetch_parcel", reply.notice_eh)?;
            debug!("receive_delivery() response: {:?}", response);
            assert!(matches!(response, ZomeCallResponse::Ok { .. }));
        }
        /// Done
        Ok(())
    }
}