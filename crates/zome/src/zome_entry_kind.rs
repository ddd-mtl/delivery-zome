use hdk::prelude::*;
use zome_delivery_types::*;
use crate::entry_kind::*;



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

///
pub(crate) fn deserialize_into_type(entry_index: EntryDefIndex, entry_bytes: AppEntryBytes) -> ExternResult<Box<dyn ZomeEntryKind>> {
   trace!("*** can_deserialize_into_type() called! ({:?})", entry_index);
   let sb = entry_bytes.into_sb();
   let entry_kind = EntryKind::from_index(&entry_index);

   match entry_kind {
      EntryKind::PubEncKey => Ok(Box::new(PubEncKey::try_from(sb.clone())?)),
      EntryKind::Path => Ok(Box::new(PathEntry::try_from(sb.clone())?)),
      EntryKind::DeliveryNotice => Ok(Box::new(DeliveryNotice::try_from(sb.clone())?)),
      EntryKind::Distribution => Ok(Box::new(Distribution::try_from(sb.clone())?)),
      EntryKind::DeliveryReceipt => Ok(Box::new(DeliveryReceipt::try_from(sb.clone())?)),
      EntryKind::DeliveryReply => Ok(Box::new(DeliveryReply::try_from(sb.clone())?)),
      EntryKind::NoticeReceived => Ok(Box::new(NoticeReceived::try_from(sb.clone())?)),
      EntryKind::ParcelReceived => Ok(Box::new(ParcelReceived::try_from(sb.clone())?)),
      EntryKind::ReplyReceived => Ok(Box::new(ReplyReceived::try_from(sb.clone())?)),
      EntryKind::PendingItem => Ok(Box::new(PendingItem::try_from(sb.clone())?)),
      EntryKind::ParcelManifest => Ok(Box::new(ParcelManifest::try_from(sb.clone())?)),
      EntryKind::ParcelChunk => Ok(Box::new(ParcelChunk::try_from(sb.clone())?)),
   }
}


/// Try to deserialize entry to given type
pub fn is_type(entry: Entry, type_candidat: EntryType) -> bool {
   trace!("*** is_type() called: {:?} == {:?} ?", type_candidat, entry);
   let res =  match entry {
      Entry::CounterSign(_data, _bytes) => unreachable!(),
      Entry::Agent(_agent_hash) => EntryType::AgentPubKey == type_candidat,
      Entry::CapClaim(_claim) => EntryType::CapClaim == type_candidat,
      Entry::CapGrant(_grant) => EntryType::CapGrant == type_candidat,
      Entry::App(entry_bytes) => {
         let mut res = false;
         if let EntryType::App(app_entry_type) = type_candidat.clone() {
            res = deserialize_into_type(app_entry_type.id(), entry_bytes).is_ok()
         }
         res
      },
   };
   trace!("*** is_type({:?}) result = {}", type_candidat, res);
   res
}

