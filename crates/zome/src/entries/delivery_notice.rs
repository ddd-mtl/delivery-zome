use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use zome_utils::*;
use crate::send_item::*;
use crate::functions::*;

impl ZomeEntry for DeliveryNotice {

   /// Delete link and signal client
   fn post_commit(&self, notice_eh: &EntryHash) -> ExternResult<()> {
      debug!("post_commit_DeliveryNotice() {:?}", notice_eh);
      /// Delete link:
      // FIXME: Find link to PendingItem that has same distribution_eh and ItemKind::DeliveryNotice
      Ok(())
   }
}