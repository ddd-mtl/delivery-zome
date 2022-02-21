use hdk::prelude::*;

use crate::{
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
    pub distribution_eh: EntryHash,
    pub sender: AgentPubKey,
    pub sender_summary_signature: Signature,
    pub parcel_summary: ParcelSummary,
}


///
pub(crate) fn post_commit_DeliveryNotice(notice_eh: &EntryHash, _notice: DeliveryNotice) -> ExternResult<()> {
    debug!("post_commit_delivery_notice() {:?}", notice_eh);

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

    // FIXME delete pending item link


    /// Done
    Ok(())
}


