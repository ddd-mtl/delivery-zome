use hdk::prelude::*;
use zome_delivery_types::*;
use crate::DeliveryGossip;


///
pub fn emit_self_signals(signals: Vec<DeliverySignalProtocol>) -> ExternResult<()> {
  if signals.is_empty() {
    return Ok(());
  }
  let signal = DeliverySignal {
    from: agent_info()?.agent_latest_pubkey,
    signal: signals,
  };
  return emit_signal(&signal);
}


///
pub fn emit_self_signal(signal: DeliverySignalProtocol) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: agent_info()?.agent_latest_pubkey,
    signal: vec![signal],
  };
  return emit_signal(&signal);
}


///
pub fn emit_gossip_signal(dg: DeliveryGossip) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: dg.from,
    signal: vec![DeliverySignalProtocol::Gossip(dg.gossip)],
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
