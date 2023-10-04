use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::{DeliveryProtocol, send_dm};


/// Notify other agents that a new Parcel has been shared publicly
#[hdk_extern]
pub fn notify_new_public_parcel(input: NotifyInput) -> ExternResult<()> {
    let msg = DeliveryProtocol::NewPublicParcel((input.timestamp, input.pr, agent_info()?.agent_latest_pubkey));
    for peer in input.peers {
        let _ = send_dm(peer, msg.clone());
    }
    Ok(())
}
