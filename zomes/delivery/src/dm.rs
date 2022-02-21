use hdk::prelude::*;
use crate::{
   dm_protocol::*,
   utils::*,
};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DmPacket {
   pub from: AgentPubKey,
   pub dm: DeliveryProtocol,
}

/// Start point for any remote call
/// WARN: Name of function must match REMOTE_ENDPOINT const value
#[hdk_extern]
pub fn receive_dm(dm_packet: DmPacket) -> ExternResult<DeliveryProtocol> {
   // let (from, dm): (AgentPubKey, DirectMessageProtocol) = dm_packet.into();
   debug!("*** receive_dm() called from {:?}", dm_packet.from);
   let response = mail::receive_dm(dm_packet.from, dm_packet.dm);
   debug!("*** receive_dm() response to send back: {:?}", response);
   Ok(response)
}

///
pub(crate) fn send_dm(destination: AgentPubKey, dm: DeliveryProtocol) -> ExternResult<DeliveryProtocol> {
   /// Pre-conditions: Don't call yourself (otherwise we get concurrency issues)
   let me = agent_info().unwrap().agent_latest_pubkey;
   if destination == me {
      /// FOR DEBUGGING ONLY?
      return error("send_dm() aborted. Can't send to self.");
   }
   /// Prepare payload
   let dm_packet = DmPacket { from: me, dm: dm.clone() };
   /// Call peer
   debug!("calling remote receive_dm() ; dm = {:?}", dm);
   let response = call_remote(
      destination,
      zome_info()?.name,
      REMOTE_ENDPOINT.to_string().into(),
      None,
      &dm_packet,
   )?;
   debug!("calling remote receive_dm() DONE ; dm = {:?}", dm);
   return match response {
       ZomeCallResponse::Ok(output) => Ok(output.decode()?),
       ZomeCallResponse::Unauthorized(_, _, _, _) => Ok(DeliveryProtocol::Failure("Unauthorized".to_string())),
       ZomeCallResponse::NetworkError(e) => Ok(DeliveryProtocol::Failure(format!("NetworkError: {:?}", e))),
       ZomeCallResponse::CountersigningSession(e) => Ok(DeliveryProtocol::Failure(format!("CountersigningSession: {:?}", e))),
   };
}
