use hdk::prelude::*;
use delivery_zome_api::entry_kind::EntryKind;
use delivery_zome_api::entries::*;
use std::str::FromStr;

pub(crate) trait ZomeEntryKind {
   fn index(&self) -> EntryDefIndex;
   fn from_index(index: &EntryDefIndex) -> Self;
   fn as_def(&self) -> EntryDef;
   fn visibility(&self) -> EntryVisibility;
   fn as_type(&self) -> EntryType;
}


impl ZomeEntryKind for EntryKind {
   ///
   fn index(&self) -> EntryDefIndex {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let id = EntryDefId::App(self.as_ref().to_string());
      debug!("EntryKind::index() def id = {:?}", id);
      let mut i = 0;
      for entry_def in entre_defs.clone() {
         debug!("entry def id: {:?} == {:?} ? {}", entry_def.id, id, entry_def.id == id);
         if entry_def.id == id {
            debug!("entry def id match. Index = {}", i);
         }
         i += 1;
      }
      let maybe_index = entre_defs.entry_def_index_from_id(id.clone());
      if let Some(index) = maybe_index {
         return index;
      }
      error!("Fatal error EntryKind::index() not found for {:?}", id);
      unreachable!()
   }

   ///
   fn from_index(index: &EntryDefIndex) -> Self {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let i: usize = index.0 as usize;
      trace!("EntryKind::from_index() i = {}", i);
      let entry_def_id = entre_defs[i].id.clone();
      if let EntryDefId::App(id) = entry_def_id {
         return Self::from_str(&id)
            .expect("Zome should have Entry with that name");
      }
      error!("Fatal error in EntryKind::from_index()");
      unreachable!()
   }

   ///
   fn as_def(&self) -> EntryDef {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let index = self.index();
      return entre_defs[index.0 as usize].clone();
   }

   ///
   fn visibility(&self) -> EntryVisibility {
      return self.as_def().visibility;
   }


   ///
   fn as_type(&self) -> EntryType {
      let app_type = AppEntryType::new(
         self.index(),
         zome_info().unwrap().id,
         self.visibility(),
      );
      EntryType::App(app_type)
   }
}



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