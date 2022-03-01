use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use zome_utils::*;
use crate::send_item::*;
use crate::functions::*;

impl ZomeEntry for DeliveryReply {
    ///
    fn post_commit(&self, reply_eh: &EntryHash) -> ExternResult<()> {
        debug!("post_commit_DeliveryReply() {:?}", reply_eh);
        debug!("self.notice_eh = {:?}", self.notice_eh.clone());
        /// Get DeliveryNotice
        let notice: DeliveryNotice = get_typed_from_eh(self.notice_eh.clone())?;
        /// Create PendingItem from DeliveryReply
        let pending_item = pack_reply(self.clone(), notice.distribution_eh.clone(), notice.sender.clone())?;
        /// Send item to recipient
        let _res = send_item(
            notice.sender,
            pending_item,
            notice.parcel_summary.distribution_strategy,
        );
        /// Try to retrieve parcel if it has been accepted
        if self.has_accepted {
            let response = call_self("fetch_parcel", self.notice_eh.clone())?;
            debug!("fetch_parcel() response: {:?}", response);
            assert!(matches!(response, ZomeCallResponse::Ok { .. }));
        }
        /// Done
        Ok(())
    }
}