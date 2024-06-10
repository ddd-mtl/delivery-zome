use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::{DeliveryGossip, emit_gossip_signal};


///
pub fn post_commit_create_PublicParcel(_sah: &SignedActionHashed, create: &Create, entry: Entry) -> ExternResult<DeliveryEntryKind> {
   debug!("post_commit_create_PublicParcel() {:?}", create.entry_hash);
   let parcel_reference = ParcelReference::try_from(entry)?;
   /// Create Anchor
   let response = call_self("link_public_parcel", create.entry_hash.clone())?;
   let _ah = decode_response::<ActionHash>(response)?;
   // /// Emit Signal
   // let dg = DeliveryGossip {
   //   from: agent_info()?.agent_latest_pubkey,
   //   gossip: DeliveryGossipProtocol::PublicParcelPublished((eh.to_owned(), sah.hashed.content.timestamp(), parcel_reference)),
   // };
   // let res = emit_gossip_signal(dg);
   // if let Err(err) = res {
   //    error!("Emit signal failed: {}", err);
   // }
   /// Done
   Ok(DeliveryEntryKind::PublicParcel(parcel_reference))
}


///
pub fn gossip_public_parcel(create_link: &CreateLink, ts: Timestamp, isCreate: bool) {
   /// Get ParcelReference
   let pp_eh = EntryHash::try_from(create_link.target_address.clone()).unwrap();
   debug!("gossip_public_parcel({}) {}", isCreate, pp_eh);
   let Ok(Some(pr_record)) = get(pp_eh.clone(), GetOptions::local())
     else { error!("Failed to get ParcelReference record"); return };
   let Ok(pr) = get_typed_from_record::<ParcelReference>(pr_record)
     else { error!("Failed to get ParcelReference"); return };
   /// Emit Signal
   let dg = DeliveryGossip {
      from: agent_info().unwrap().agent_latest_pubkey,
      gossip: if isCreate {
         DeliveryGossipProtocol::PublicParcelPublished((pp_eh, ts, pr))
      } else {
         DeliveryGossipProtocol::PublicParcelUnpublished((pp_eh, ts, pr))
      },
   };
   let res = emit_gossip_signal(dg);
   if let Err(e) = res {
      error!("Failed to get emit gossip signal: {:?}", e);
   }
}
