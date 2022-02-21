use hdk::prelude::*;

use crate::{
   entries::*,
};

pub type AppType = (ZomeName, EntryDefId);


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ParcelKind {
   AppEntry(AppType),
   Manifest,
   //Acknowledgement,
   //Notification,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParcelSummary {
   pub size: usize,
   pub reference: ParcelReference,
}

pub enum ParcelReference {
   AppEntry((AppType, EntryHash)),
   Manifest(EntryHash),
}

impl ParcelReference {
   pub fn entry_address(&self) -> EntryHash {
      match self {
         ParcelReference::Manifest(eh) => eh.clone(),
         ParcelReference::AppEntry((_, eh)) => eh.clone(),
      }
   }
}

