use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::*;


///
pub fn send_dm(destination: AgentPubKey, msg: DeliveryProtocol) -> ExternResult<DeliveryProtocol> {
   /// Pre-conditions: Don't call yourself (otherwise we get concurrency issues)
   let me = agent_info()?.agent_latest_pubkey;
   if destination == me {
      return error("send_dm() aborted. Can't send to self.");
   }
   /// Prepare payload
   let dm_packet = DirectMessage { from: me, msg: msg.clone() };
   /// Call peer
   debug!("calling remote receive_dm()");
   trace!("dm = '{}'", msg);
   let response = call_remote(
      destination,
      zome_info()?.name,
      REMOTE_ENDPOINT.to_string().into(),
      None,
      &dm_packet,
   )?;
   debug!("calling remote receive_dm() DONE");
   trace!("msg = '{}' ; Response: {:?}", msg, response);
   return match response {
       ZomeCallResponse::Ok(output) => Ok(output.decode().map_err(|e| wasm_error!(WasmErrorInner::Serialize(e)))?),
       ZomeCallResponse::Unauthorized(_, _, _, _, _) => Ok(DeliveryProtocol::Failure("Unauthorized".to_string())),
       ZomeCallResponse::NetworkError(e) => Ok(DeliveryProtocol::Failure(format!("NetworkError: {:?}", e))),
       ZomeCallResponse::CountersigningSession(e) => Ok(DeliveryProtocol::Failure(format!("CountersigningSession: {:?}", e))),
   };
}



///
pub fn send_dm_signal(destination: AgentPubKey, msg: DeliveryProtocol) -> ExternResult<()> {
   /// Pre-conditions: Don't call yourself (otherwise we get concurrency issues)
   let me = agent_info()?.agent_latest_pubkey;
   if destination == me {
      return error("send_dm() aborted. Can't send to self.");
   }
   /// Prepare payload
   let dm_packet = DirectMessage { from: me, msg: msg.clone() };
   /// Call peer
   debug!("calling remote recv_remote_signal()");
   trace!("dm = '{}'", msg);
   remote_signal(
      &dm_packet,
      vec![destination],
   )?;
   debug!("calling remote recv_remote_signal() DONE");
   Ok(())
}