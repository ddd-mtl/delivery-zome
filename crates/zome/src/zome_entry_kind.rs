use hdk::prelude::*;
use zome_delivery_types::*;
use crate::entry_kind::*;


///
fn can_deserialize_into_type(entry_type_index: EntryDefIndex, entry_bytes: AppEntryBytes) -> bool {
   trace!("*** can_deserialize_into_type() called! ({:?})", entry_type_index);
   let sb = entry_bytes.into_sb();
   let entry_kind = EntryKind::from_index(&entry_type_index);

   match entry_kind {
      EntryKind::PubEncKey => PubEncKey::try_from(sb.clone()).is_ok(),
      EntryKind::Path => PathEntry::try_from(sb.clone()).is_ok(),
      EntryKind::DeliveryNotice => DeliveryNotice::try_from(sb.clone()).is_ok(),
      EntryKind::DeliveryReceipt => DeliveryReceipt::try_from(sb.clone()).is_ok(),
      EntryKind::DeliveryReply => DeliveryReply::try_from(sb.clone()).is_ok(),
      EntryKind::Distribution => Distribution::try_from(sb.clone()).is_ok(),
      EntryKind::NoticeReceived => NoticeReceived::try_from(sb.clone()).is_ok(),
      EntryKind::ParcelReceived => ParcelReceived::try_from(sb.clone()).is_ok(),
      EntryKind::ReplyReceived => ReplyReceived::try_from(sb.clone()).is_ok(),
      EntryKind::PendingItem => PendingItem::try_from(sb.clone()).is_ok(),
      EntryKind::ParcelManifest => ParcelManifest::try_from(sb.clone()).is_ok(),
      EntryKind::ParcelChunk => ParcelChunk::try_from(sb.clone()).is_ok(),
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
            res = can_deserialize_into_type(app_entry_type.id(), entry_bytes)
         }
         res
      },
   };
   trace!("*** is_type({:?}) result = {}", type_candidat, res);
   res
}