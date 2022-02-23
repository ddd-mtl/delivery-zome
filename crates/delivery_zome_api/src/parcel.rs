use hdk::prelude::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

   pub fn entry_def_id(&self) -> EntryDefId {
      match self {
         ParcelReference::Manifest(_) => EntryDefId::App("ParcelManifest".to_string()),
         ParcelReference::AppEntry((app_type, _)) => app_type.1.clone(),
      }
   }
}

