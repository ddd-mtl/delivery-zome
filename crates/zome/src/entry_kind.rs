use hdk::prelude::*;

use std::str::FromStr;

use std::convert::AsRef;
use strum_macros::AsRefStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//use zome_delivery_types::*;

/// Listing all Entry kinds for this Zome
#[derive(AsRefStr, EnumIter, Ordinalize, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryKind {
   Path,
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
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
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
   pub fn index(&self) -> EntryDefIndex {
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
      unreachable!()
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



