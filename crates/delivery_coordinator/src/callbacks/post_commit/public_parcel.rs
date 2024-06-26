use hdk::prelude::*;
use zome_utils::*;
// use zome_delivery_types::*;
// use crate::*;


///
pub fn post_commit_create_PublicParcel(_sah: &SignedActionHashed, eh: &EntryHash, _entry: Entry) -> ExternResult<()> {
   debug!("post_commit_create_PublicParcel() create: {}", eh);
   /// Create Anchor
   let response = call_self("link_public_parcel", eh.clone())?;
   let _ah = decode_response::<ActionHash>(response)?;
   /// Done
   Ok(())
}


// /// TODO: Change this. Should not gossip self
// pub fn self_gossip_public_parcel(create_link: &CreateLink, ts: Timestamp, isCreate: bool) {
//    /// Get ParcelReference
//    let pr_eh = EntryHash::try_from(create_link.target_address.clone()).unwrap();
//    let base_eh = EntryHash::try_from(create_link.base_address.clone()).unwrap();
//    debug!("gossip_public_parcel({}) {} | base: {}", isCreate, pr_eh, base_eh);
//    let maybe = get(pr_eh.clone(), GetOptions::local());
//    if let Err(e) = maybe {
//       error!("get() ParcelReference record failed: {:?}", e);
//       return;
//    }
//    let Some(pr_record) = maybe.unwrap()
//      else { error!("ParcelReference record not found. It might be the initial anchor link."); return };
//    let Ok(pr) = get_typed_from_record::<ParcelReference>(pr_record)
//      else { error!("Failed to convert entry to ParcelReference"); return };
//    /// Emit Signal
//    let gossip = if isCreate {
//       DeliveryTipProtocol::PublicParcelPublished((pr_eh, ts, pr))
//    } else {
//       DeliveryTipProtocol::PublicParcelUnpublished((pr_eh, ts, pr))
//    };
//    let res = emit_self_gossip_signal(gossip);
//    if let Err(e) = res {
//       error!("Failed to get emit gossip signal: {:?}", e);
//    }
// }
