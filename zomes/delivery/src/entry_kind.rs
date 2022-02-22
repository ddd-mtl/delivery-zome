use hdk::prelude::*;

use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
//use strum::EnumProperty;

use crate::{
   utils::*,
};
use crate::entries::*;
use crate::entries::parcel_chunk::ParcelChunk;
use crate::entries::pub_enc_key::*;

entry_defs![
   /// -- PubEncKey
   PubEncKey::entry_def(),
   /// -- Delivery
   DeliveryNotice::entry_def(),
   DeliveryReceipt::entry_def(),
   DeliveryReply::entry_def(),
   Distribution::entry_def(),
   NoticeReceived::entry_def(),
   ParcelReceived::entry_def(),
   ReplyReceived::entry_def(),
   PendingItem::entry_def(),
   ParcelManifest::entry_def(),
   ParcelChunk::entry_def(),
   /// -- Other
   PathEntry::entry_def()
];


///
fn can_deserialize_into_type(entry_type_index: EntryDefIndex, entry_bytes: AppEntryBytes) -> bool {
   trace!("*** can_deserialize() called! ({:?})", entry_type_index);
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


/// Listing all Entry kinds for this DNA
/// !! Visibility prop value must match hdk_entry visibility !!
#[derive(AsStaticStr, EnumIter, Ordinalize, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryKind {
   #[strum]
   Path,
   #[strum]
   PubEncKey,
   #[strum]
   DeliveryNotice,
   #[strum]
   DeliveryReceipt,
   #[strum]
   DeliveryReply,
   #[strum]
   Distribution,
   #[strum]
   NoticeReceived,
   #[strum]
   ParcelReceived,
   #[strum]
   ReplyReceived,
   #[strum]
   PendingItem,
   #[strum]
   ParcelManifest,
   #[strum]
   ParcelChunk,
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
      for entry_kind in EntryKind::iter() {
         if input == entry_kind.into() {
            return Ok(entry_kind);
         }
      }
      Err(())
   }
}


impl EntryKind {
   ///
   pub fn index(&self) -> EntryDefIndex {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let maybe_index = entre_defs.entry_def_index_from_id(EntryDefId::App(self.into()));
      if let Some(index) = maybe_index {
         return index;
      }
      error!("Fatal error EntryKind::index() not found.");
      unreachable!()
   }

   ///
   pub fn as_def(&self) -> EntryDef {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let index = self.index();
      return entre_defs[index.into()].clone();
   }

   ///
   pub fn visibility(&self) -> EntryVisibility {
      return self.as_def().visibility;
   }


   ///
   pub fn as_type(&self) -> EntryType {
      let app_type = AppEntryType::new(
         self.index(),
   zome_info().unwrap().id,
         self.visibility(),
      );
      EntryType::App(app_type)
   }
}




/// Get EntryType out of an Entry & EntryHash
pub fn determine_entry_type(eh: EntryHash, entry: &Entry) -> ExternResult<EntryType> {
   Ok(match entry {
      Entry::Agent(_agent_hash) => EntryType::AgentPubKey,
      Entry::CapClaim(_claim) => EntryType::CapClaim,
      Entry::CapGrant(_grant) => EntryType::CapGrant,
      Entry::App(_entry_bytes) => get_entry_type_from_eh(eh)?,
      Entry::CounterSign(_data, _bytes) => unreachable!(),
   })
}

/// Try to deserialize entry to given type
pub(crate) fn is_type(entry: Entry, type_candidat: EntryType) -> bool {
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


