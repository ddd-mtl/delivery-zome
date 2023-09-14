//! Delivery related types and states

use hdi::prelude::*;
use crate::ParcelDescription;


//const MANIFEST_ENTRY_NAME: &'static str = "ParcelManifest";


// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct OutgoingDeliveryItem {
//    pub state: DeliveryState,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct IncomingDeliveryItem {
//    pub hh: ActionHash,
//    pub author: AgentPubKey,
//    //pub parcel: Parcel,
//    pub state: NoticeState,
//    // pub delivery_states: Map<AgentPubKey, DeliveryState>
//    pub recipients: Vec<AgentPubKey>,
//    pub send_date: i64,
// }


/// State of a single delivery of an item to a unique recipient
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeliveryState {
   /// Initial state ; Distribution Entry committed
   Unsent,
   /// There is a link to a PendingItem entry holding the DeliveryNotice and the inbox link is alive
   PendingNotice,
   /// NoticeDelivered committed (DeleteLink entry found)
   NoticeDelivered,
   /// Negative ReplyAck committed
   ParcelRefused,
   /// Positve ReplyAck committed
   ParcelAccepted,
   /// There is a link to a PendingItem entry holding the Parcel and inbox link is alive
   PendingParcel,
   /// ReceptionAck committed (DeleteLink entry found)
   ParcelDelivered,
}

/// Possible states of an OutMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DistributionState {
   /// Initial state ; Distribution committed
   Unsent,
   /// (yellow) All deliveries have been sent (no Unsent state)
   AllNoticesSent,
   /// (black) All notices have been sent (no Unsent or PendingNotice state)
   AllNoticeReceived,
   /// (blue) All deliveries have ParcelRefused, ParcelAccepted or PendingParcel state
   AllRepliesReceived,
   /// (green) All deliveries have ParcelRefused or ParcelDelivered state
   AllAcceptedParcelsReceived,
   /// (red) Delete entry commited
   Deleted,
}

/// Possible states of a DeliveryNotice entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NoticeState {
   /// DeliveryNotice committed
   Unreplied,
   /// ReceptionConfirmation(yes) committed
   Accepted,
   /// ReceptionConfirmation(no) committed
   Refused,
   /// ReceptionProof committed
   Received,
   /// ??? Parcel deleted ???
   Deleted,
}

// /// Information for commiting Entry
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct EntryReference {
//    pub entry_index: EntryDefIndex,
//    pub visibility: EntryVisibility,
// }
//
// /// Informantion about where the data is from
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct ManifestReference {
//    pub data_type: String,
// }


/// Shared data between a Distribution and a DeliveryNotice
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeliverySummary {
   pub distribution_strategy: DistributionStrategy,
   pub parcel_description: ParcelDescription,
}


/// A Parcel is a generic Entry or a ParcelManifest
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParcelReference {
   pub eh: EntryHash,
   pub zome_origin: ZomeName,
   pub visibility: EntryVisibility,
   pub kind_info: ParcelKind,
}

impl ParcelReference {
   pub fn entry_integrity_zome_name(&self) -> ZomeName {
      match self.kind_info {
         ParcelKind::Manifest(_) => crate::DELIVERY_INTERGRITY_ZOME_NAME.to_string().into(),
         ParcelKind::AppEntry(_) => self.zome_origin.clone(),
      }
   }

   pub fn zome_index(&self) -> ZomeIndex {
      let izn = self.entry_integrity_zome_name();
      /// Search for zome_index
      let mut i: u8 = 0;
      for zome_name in dna_info().unwrap().zome_names {
         if zome_name == izn {
            break;
         }
         i += 1;
      }
      if i == dna_info().unwrap().zome_names.len() as u8 {
         debug!("Zome index not found for {:?}", izn);
         panic!("ZOME INDEX NOT FOUND");
      }
      /// Return found value
      ZomeIndex::from(i)
   }
}


/// A Parcel is a generic Entry or a ParcelManifest
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ParcelKind {
   /// Any Entry type
   AppEntry(EntryDefIndex),
   /// A ParcelManifest
   Manifest(String), // data_type
}

impl ParcelKind {
   pub fn entry_index(&self) -> EntryDefIndex {
      match self {
         ParcelKind::AppEntry(entry_index) => entry_index.to_owned(),
         ParcelKind::Manifest(_) => EntryDefIndex::from(6), // FIXME should not be hardcoded: DeliveryEntryTypes::ParcelManifest
      }
   }
}


///
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