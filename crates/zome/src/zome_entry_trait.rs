use hdk::prelude::*;
use zome_delivery_types::*;


pub trait ZomeEntry {
   fn validate(&self, _maybe_package: Option<ValidationPackage>) -> ExternResult<ValidateCallbackResult> {
      Ok(ValidateCallbackResult::Valid)
   }
   fn post_commit(&self, _eh: &EntryHash) -> ExternResult<()> { Ok(()) }

   //fn commit();
}

impl ZomeEntry for PubEncKey {}
impl ZomeEntry for PathEntry {}
impl ZomeEntry for DeliveryNotice {}
//impl ZomeEntryKind for Distribution {}
impl ZomeEntry for DeliveryReceipt {}
//impl ZomeEntry for DeliveryReply {}
impl ZomeEntry for NoticeReceived {}
//impl ZomeEntry for ParcelReceived {}
impl ZomeEntry for ReplyReceived {}
impl ZomeEntry for PendingItem {}
//impl ZomeEntry for ParcelManifest {}
//impl ZomeEntry for ParcelChunk {}


