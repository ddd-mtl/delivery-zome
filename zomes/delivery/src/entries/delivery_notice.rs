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
#[hdk_entry(id = "DeliveryNotice", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
    pub parcel_summary: ParcelSummary,
    pub distribution_eh: EntryHash,
    pub sender: AgentPubKey,
    pub sender_summary_signature: Signature,
}


///
pub(crate) fn post_commit_DeliveryNotice(eh: &EntryHash, notice: DeliveryNotice) -> ExternResult<()> {
    debug!("post_commit_delivery_notice() {:?}", eh);

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

    let signature = sign(agent_info()?.agent_latest_pubkey, notice)?;

    /// Create PendingItem
    let pending_item = PendingItem::from_notification(
        notice.clone(),
        distribution_eh.clone(),
        recipient.clone(),
    )?;

    /// Send confirmation to sender
    send_item(
        notice.sender.clone(),
        notice.distribution_eh.clone(),
        pending_item,
        signature,
    )?;

    /// Done
    Ok(())
}


