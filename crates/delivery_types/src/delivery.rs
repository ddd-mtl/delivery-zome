//! Delivery related types and states

use hdi::prelude::*;


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
   /// Negative ReplyReceived committed
   ParcelRefused,
   /// positve ReplyReceived committed
   ParcelAccepted,
   /// There is a link to a PendingItem entry holding the Parcel and inbox link is alive
   PendingParcel,
   /// DeliveryReceipt committed (DeleteLink entry found)
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
   /// ParcelReceived committed
   Received,
   /// ??? Parcel deleted ???
   Deleted,
}



/// Shared data between a Distribution and a DeliveryNotice
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeliverySummary {
   pub distribution_strategy: DistributionStrategy,
   pub parcel_size: usize,
   pub parcel_reference: ParcelReference,
}

/// A Parcel is a generic Entry or a ParcelManifest
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ParcelReference {
   /// Any Entry type
   AppEntry(ZomeName, EntryDefIndex, EntryHash, EntryVisibility),
   /// A ParcelManifest
   Manifest(EntryHash),
}

impl ParcelReference {
   pub fn entry_address(&self) -> EntryHash {
      match self {
         ParcelReference::Manifest(eh) => eh.clone(),
         ParcelReference::AppEntry(_,_, eh,_) => eh.clone(),
      }
   }

   pub fn entry_index(&self) -> EntryDefIndex {
      match self {
         ParcelReference::Manifest(_) => EntryDefIndex::from(6), // FIXME should not be hardcoded
         ParcelReference::AppEntry(_, id, _, _) => id.to_owned(),
      }
   }

   pub fn entry_zome_name(&self) -> ZomeName {
      match self {
         ParcelReference::Manifest(_) => crate::DELIVERY_ZOME_NAME.to_string().into(),
         ParcelReference::AppEntry(zn,_, _, _) => zn.clone(),
      }
   }

   pub fn entry_visibility(&self) -> EntryVisibility {
      match self {
         ParcelReference::Manifest(_) => EntryVisibility::Private,
         ParcelReference::AppEntry(_,_, _, viz) => viz.clone(),
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