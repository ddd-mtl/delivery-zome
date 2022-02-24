use hdk::prelude::*;
use zome_delivery_types::*;
//use crate::entry_kind::*;



pub trait ZomeEntryKind {
   fn validate(&self, _maybe_package: Option<ValidationPackage>) -> ExternResult<ValidateCallbackResult> {
      Ok(ValidateCallbackResult::Valid)
   }
   fn post_commit(&self, _eh: &EntryHash) -> ExternResult<()> { Ok(()) }
   //fn commit();
}

impl ZomeEntryKind for PubEncKey {}
impl ZomeEntryKind for PathEntry {}
impl ZomeEntryKind for DeliveryNotice {}
//impl ZomeEntryKind for Distribution {}
impl ZomeEntryKind for DeliveryReceipt {}
impl ZomeEntryKind for DeliveryReply {}
impl ZomeEntryKind for NoticeReceived {}
impl ZomeEntryKind for ParcelReceived {}
impl ZomeEntryKind for ReplyReceived {}
impl ZomeEntryKind for PendingItem {}
impl ZomeEntryKind for ParcelManifest {}
impl ZomeEntryKind for ParcelChunk {}


