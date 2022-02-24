use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use crate::utils::*;
use crate::send_item::*;
use crate::functions::*;

impl ZomeEntry for ParcelReceived {
   ///
   fn post_commit(&self, receipt_eh: &EntryHash) -> ExternResult<()> {
      debug!("post_commit_ParcelReceived() {:?}", receipt_eh);
      /// Get DeliveryNotice
      let notice: DeliveryNotice = get_typed_from_eh(self.notice_eh.clone())?;
      /// Sign Item
      //let signature = sign(agent_info()?.agent_latest_pubkey, reception.clone())?;
      /// Create PendingItem
      let pending_item = pack_reception(
         self.clone(),
         notice.distribution_eh.clone(),
         notice.sender.clone(),
      )?;
      /// Send it to recipient
      let _ = send_item(
         notice.sender,
         //notice.distribution_eh.clone(),
         pending_item,
         //signature,
      )?;
      /// Done
      Ok(())
   }
}