use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::{DeliveryProtocol, send_dm_signal};


/// Notify other agents that a new Parcel has been shared publicly
#[hdk_extern]
pub fn notify_new_public_parcel(input: NotifyInput) -> ExternResult<()> {
    debug!("peer count: {}", input.peers.len());
    std::panic::set_hook(Box::new(zome_panic_hook));
    let msg = DeliveryProtocol::PublicParcelPublished((input.timestamp, input.pr, agent_info()?.agent_latest_pubkey));
    for peer in input.peers {
        let _ = send_dm_signal(peer, msg.clone());
    }
    debug!("DONE");
    Ok(())
}
