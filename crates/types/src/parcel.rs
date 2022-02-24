use hdk::prelude::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParcelSummary {
   pub size: usize,
   pub reference: ParcelReference,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ParcelReference {
   /// Any Entry type
   AppEntry(ZomeName, EntryDefId, EntryHash),
   /// A ParcelManifest
   Manifest(EntryHash),
}

impl ParcelReference {
   pub fn entry_address(&self) -> EntryHash {
      match self {
         ParcelReference::Manifest(eh) => eh.clone(),
         ParcelReference::AppEntry(_,_, eh) => eh.clone(),
      }
   }

   pub fn entry_def_id(&self) -> EntryDefId {
      match self {
         ParcelReference::Manifest(_) => EntryDefId::App("ParcelManifest".to_string()),
         ParcelReference::AppEntry(_, id, _) => id.to_owned(),
      }
   }
}

