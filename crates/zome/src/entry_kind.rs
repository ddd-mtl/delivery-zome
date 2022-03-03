use hdk::prelude::*;

use std::str::FromStr;
use std::convert::AsRef;
use strum_macros::{AsRefStr, EnumIter};
use strum::IntoEnumIterator;

use crate::zome_entry_trait::*;
use zome_delivery_types::*;


/// Listing all Entry kinds for this Zome
#[derive(AsRefStr, EnumIter, Ordinalize, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryKind {
   PubEncKey,
   DeliveryNotice,
   DeliveryReceipt,
   DeliveryReply,
   Distribution,
   NoticeReceived,
   ParcelReceived,
   ReplyReceived,
   PendingItem,
   ParcelManifest,
   ParcelChunk,
   #[strum(serialize = "hdk.path_entry")]
   Path,
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
      trace!("EntryKind::from_str({}) called...", input);
      for entry_kind in EntryKind::iter() {
         //let entry_kind = EntryKind::from_ordinal(ordinal);
         if input == entry_kind.as_ref() {
            return Ok(entry_kind);
         }
      }
      Err(())
   }
}


impl EntryKind {
   ///
   pub fn into_zome_entry(&self, entry_bytes: AppEntryBytes) -> ExternResult<Box<dyn ZomeEntry>> {
      let sb = entry_bytes.into_sb();
      match self {
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

   ///
   pub fn index(&self) -> EntryDefIndex {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let id = EntryDefId::App(self.as_ref().to_string());
      trace!("EntryKind::index() def id = {:?}", id);
      let mut i = 0;
      for entry_def in entre_defs.clone() {
         trace!("entry def id: {:?} == {:?} ? {}", entry_def.id, id, entry_def.id == id);
         if entry_def.id == id {
            trace!("entry def id match. Index = {}", i);
         }
         i += 1;
      }
      let maybe_index = entre_defs.entry_def_index_from_id(id.clone());
      if let Some(index) = maybe_index {
         return index;
      }
      error!("Fatal error EntryKind::index() not found for {:?}", id);
      unreachable!("EntryKind::index() not found")
   }

   ///
   pub fn from_index(index: &EntryDefIndex) -> Self {
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
      unreachable!("Fatal error in EntryKind::from_index()")
   }

   ///
   pub fn as_def(&self) -> EntryDef {
      let entre_defs = zome_info()
         .expect("Zome should be operational")
         .entry_defs;
      let index = self.index();
      return entre_defs[index.0 as usize].clone();
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



