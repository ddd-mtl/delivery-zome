use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

use crate::zome_entry_trait::*;
use crate::send_item::*;
use crate::functions::*;


impl ZomeEntry for ParcelReceived {
   ///
   fn post_commit(&self, receipt_eh: &EntryHash) -> ExternResult<()> {
      debug!("post_commit_ParcelReceived() {:?}", receipt_eh);
      /// Get DeliveryNotice
      let notice: DeliveryNotice = get_typed_from_eh(self.notice_eh.clone())?;
      /// Create PendingItem
      let pending_item = pack_reception(
         self.clone(),
         notice.distribution_eh.clone(),
         notice.sender.clone(),
      )?;
      /// Send it to recipient
      let _ = send_item(
         notice.sender,
         pending_item,
         notice.summary.distribution_strategy,
      )?;
      /// Done
      Ok(())
   }
}