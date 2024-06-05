use hdk::prelude::*;
use zome_utils::zome_panic_hook;
use crate::emit_gossip_signal;
use crate::broadcast_gossip::DeliveryGossip;


///
#[hdk_extern]
fn recv_remote_signal(gossip_signal: SerializedBytes) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("START - {}", gossip_signal.bytes().len());
   /// Unpack gossip
   let dg: DeliveryGossip = DeliveryGossip::try_from(gossip_signal)
      .map_err(|e| wasm_error!(WasmErrorInner::Serialize(e)))?;
   debug!("dg: {:?}", dg);
   /// Emit signal
   let res = emit_gossip_signal(dg);
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}
