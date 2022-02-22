use hdk::prelude::*;
//use serde::de::Unexpected::Str;
use crate::{dm_protocol::*, utils::*, constants::*, DirectMessage};


///
pub(crate) fn send_dm(destination: AgentPubKey, msg: DeliveryProtocol) -> ExternResult<DeliveryProtocol> {
   /// Pre-conditions: Don't call yourself (otherwise we get concurrency issues)
   let me = agent_info().unwrap().agent_latest_pubkey;
   if destination == me {
      /// FOR DEBUGGING ONLY?
      return error("send_dm() aborted. Can't send to self.");
   }
   /// Prepare payload
   let dm_packet = DirectMessage { from: me, msg: msg.clone() };
   /// Call peer
   debug!("calling remote receive_dm() ; dm = {:?}", msg);
   let response = call_remote(
      destination,
      zome_info()?.name,
      REMOTE_ENDPOINT.to_string().into(),
      None,
      &dm_packet,
   )?;
   debug!("calling remote receive_dm() DONE ; msg = {:?}", msg);
   return match response {
       ZomeCallResponse::Ok(output) => Ok(output.decode()?),
       ZomeCallResponse::Unauthorized(_, _, _, _) => Ok(DeliveryProtocol::Failure("Unauthorized".to_string())),
       ZomeCallResponse::NetworkError(e) => Ok(DeliveryProtocol::Failure(format!("NetworkError: {:?}", e))),
       ZomeCallResponse::CountersigningSession(e) => Ok(DeliveryProtocol::Failure(format!("CountersigningSession: {:?}", e))),
   };
}
