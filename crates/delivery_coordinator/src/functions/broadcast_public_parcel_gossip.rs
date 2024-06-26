// use hdk::prelude::*;
// use zome_utils::*;
//
// use zome_delivery_types::*;
// use crate::{broadcast_gossip};


// /// Notify other agents that a Public Parcel has been shared or removed publicly
// #[hdk_extern]
// pub fn broadcast_public_parcel_gossip(input: BroadcastInput) -> ExternResult<()> {
//     debug!("peer count: {}", input.peers.len());
//     std::panic::set_hook(Box::new(zome_panic_hook));
//     let eh = hash_entry(input.pr.clone())?;
//     let tuple = (eh, input.timestamp, input.pr);
//     let msg = if input.removed {
//         DeliveryTipProtocol::PublicParcelUnpublished(tuple)
//     } else {
//         DeliveryTipProtocol::PublicParcelPublished(tuple)
//     };
//     broadcast_gossip(input.peers, msg.clone())?;
//     debug!("DONE");
//     Ok(())
// }
