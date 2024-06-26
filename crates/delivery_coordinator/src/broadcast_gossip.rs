// use hdk::prelude::*;
// //use zome_utils::*;
// use zome_delivery_types::*;
//
//
// ///
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
// pub struct DeliveryGossip {
//   pub from: AgentPubKey,
//   pub gossip: DeliveryTipProtocol,
// }
//
//
// ///
// pub fn broadcast_gossip(destinations: Vec<AgentPubKey>, gossip: DeliveryTipProtocol) -> ExternResult<()> {
//   /// Pre-conditions: Don't call yourself (otherwise we get concurrency issues)
//   let me = agent_info()?.agent_latest_pubkey;
//   let dests = destinations.into_iter().filter(|agent| agent != &me).collect();
//   /// Prepare payload
//   let gossip_packet = DeliveryGossip { from: me, gossip: gossip.clone() };
//   /// Signal peers
//   debug!("calling remote recv_remote_signal() to {:?}", dests);
//   trace!("gossip = '{}'", gossip);
//   send_remote_signal(
//     ExternIO::encode(gossip_packet).unwrap(),
//     dests,
//   )?;
//   trace!("calling remote recv_remote_signal() DONE");
//   Ok(())
// }
