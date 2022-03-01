use hdk::prelude::*;
use crate::DELIVERY_ZOME_NAME;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParcelSummary {
   pub size: usize,
   pub distribution_strategy: DistributionStrategy,
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

   pub fn entry_zome_name(&self) -> ZomeName {
      match self {
         ParcelReference::Manifest(_) => DELIVERY_ZOME_NAME.to_string().into(),
         ParcelReference::AppEntry(zn,_, _) => zn.clone(),
      }
   }
}


#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DistributionStrategy {
   /// DM first, DHT otherwise
   NORMAL,
   // /// Publish to DHT unencrypted,
   // PUBLIC,
   /// Only via DM no DHT
   DM_ONLY,
   /// Encrypt to recipients on DHT, no DM
   DHT_ONLY,
}


impl DistributionStrategy {
   pub fn can_dm(&self) -> bool {
      match self {
         Self::NORMAL => true,
         Self::DHT_ONLY => false,
         Self::DM_ONLY => true,
      }
   }
   pub fn can_dht(&self) -> bool {
      match self {
         Self::NORMAL => true,
         Self::DHT_ONLY => true,
         Self::DM_ONLY => false,
      }
   }
}