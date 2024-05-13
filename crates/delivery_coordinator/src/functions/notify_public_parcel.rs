use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::{DeliveryProtocol, send_dm_signal};


/// Notify other agents that a Public Parcel has been shared or removed publicly
#[hdk_extern]
pub fn notify_public_parcel(input: NotifyInput) -> ExternResult<()> {
    debug!("peer count: {}", input.peers.len());
    std::panic::set_hook(Box::new(zome_panic_hook));
    let eh = hash_entry(input.pr.clone())?;
    let tuple = (eh, input.timestamp, input.pr, agent_info()?.agent_latest_pubkey);
    let msg = if input.removed {
        DeliveryProtocol::PublicParcelRemoved(tuple)
    } else {
        DeliveryProtocol::PublicParcelPublished(tuple)
    };
    for peer in input.peers {
        let _ = send_dm_signal(peer, msg.clone());
    }
    debug!("DONE");
    Ok(())
}
