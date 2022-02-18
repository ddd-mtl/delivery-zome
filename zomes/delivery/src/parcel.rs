
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
pub struct ParcelDescription {
   pub size: usize,
   pub parcel: Parcel,
}

pub enum Parcel {
   AppEntry((AppType, EntryHash)),
   Package(EntryHash),
}

impl Parcel {
   pub fn entry_address(&self) -> EntryHash {
      match self {
         Parcel::Package(eh) => eh.clone(),
         Parcel::AppEntry((_, eh)) => eh.clone(),
      }
   }
}

