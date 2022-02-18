use hdk::prelude::*;

use crate::{
    ItemMessage,
    utils::*,
    send_item::*,
    parcel::*,
};
use crate::entries::*;
use crate::entries::pub_enc_key::*;


/// Entry representing a received Manifest
#[hdk_entry(id = "DeliveryNotification", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotification {
    pub description: ParcelDescription,
    pub sender: AgentPubKey,
    pub sender_description_signature: Signature,
    pub sender_distribution_eh: EntryHash,
}


///
pub(crate) fn post_commit_delivery_notification(eh: &EntryHash, notification: DeliveryNotification) -> ExternResult<()> {
    debug!("post_commit_delivery_notification() {:?}", eh);

    // /// Emit signal
    // let item = MailItem {
    //     hh: maybe_hh.unwrap(),
    //     author: from.clone(),
    //     mail: msg.description.clone(),
    //     state: MailState::In(IncomingDeliveryState::ManifestReceived),
    //     bcc: Vec::new(),
    //     date: snapmail_now() as i64, // FIXME
    // };
    // let res = emit_signal(&SignalProtocol::ReceivedReceptionRequest(item));
    // if let Err(err) = res {
    //     error!("Emit signal failed: {}", err);
    // }

    let signature = sign(agent_info()?.agent_latest_pubkey, notification)?;

    /// Create PendingItem
    let pending_item = PendingItem::from_notification(
        notification.clone(),
        distribution_eh.clone(),
        recipient.clone(),
    )?;

    /// Send confirmation to sender
    send_item(
        notification.sender.clone(),
        notification.sender_distribution_eh.clone(),
        pending_item,
        signature,
    )?;

    /// Done
    Ok(())
}


