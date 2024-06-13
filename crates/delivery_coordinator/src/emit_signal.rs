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
    pulses: signals,
  };
  return emit_signal(&signal);
}


///
pub fn emit_self_signal(signal: DeliverySignalProtocol) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: agent_info()?.agent_latest_pubkey,
    pulses: vec![signal],
  };
  return emit_signal(&signal);
}


///
pub fn emit_gossip_signal(dg: DeliveryGossip) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: dg.from,
    pulses: vec![DeliverySignalProtocol::Gossip(dg.gossip)],
  };
  return emit_signal(&signal);
}


///
pub fn emit_self_gossip_signal(gossip: DeliveryGossipProtocol) -> ExternResult<()> {
  let signal = DeliverySignal {
    from: agent_info()?.agent_latest_pubkey,
    pulses: vec![DeliverySignalProtocol::Gossip(gossip)],
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


///
pub fn emit_entry_signal(state: EntryStateChange, create: &Create, kind: DeliveryEntryKind) -> ExternResult<()> {
  let dsp = entry_signal(state, create, kind);
  return emit_self_signal(dsp);
}


///
pub fn entry_signal(state: EntryStateChange, create: &Create, kind: DeliveryEntryKind) -> DeliverySignalProtocol {
  let info = EntryInfo {
    hash: AnyDhtHash::from(create.entry_hash.clone()),
    ts: create.timestamp,
    author: create.author.clone(),
    state,
  };
  DeliverySignalProtocol::Entry((info, kind))
}


///
pub fn entry_signal_ah(state: EntryStateChange, create: &Create, kind: DeliveryEntryKind, ah: ActionHash) -> DeliverySignalProtocol {
  let info = EntryInfo {
    hash: AnyDhtHash::from(ah),
    ts: create.timestamp,
    author: create.author.clone(),
    state,
  };
  DeliverySignalProtocol::Entry((info, kind))
}
