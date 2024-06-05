use hdk::prelude::*;
use zome_delivery_types::*;
use crate::DeliveryGossip;


///
pub fn emit_self_signal(signal: SignalProtocol) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: agent_info()?.agent_latest_pubkey,
    signal: signal,
  };
  return emit_signal(&signal);
}


///
pub fn emit_gossip_signal(dg: DeliveryGossip) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: dg.from,
    signal: SignalProtocol::Gossip(dg.gossip)
  };
  return emit_signal(&signal);
}


///
pub fn emit_system_signal(sys: SystemSignalProtocol) -> ExternResult<()> {
  let signal = SystemSignal {
    System: sys,
  };
  return emit_signal(&signal);
}
