use hdk::prelude::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
#[hdk_extern]
fn recv_remote_signal(dm_signal: SerializedBytes) -> ExternResult<()> {
   debug!("START - {}", dm_signal.bytes().len());
   let dm: DirectMessage = DirectMessage::try_from(dm_signal)
      .map_err(|e| wasm_error!(WasmErrorInner::Serialize(e)))?;
   debug!("dm: {:?}", dm);
   let res = receive_delivery_dm(dm)?;
   debug!("res: {:?}", res);
   Ok(())
}
