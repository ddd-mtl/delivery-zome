use hdk::prelude::*;

use crate::{
    ReceptionRequestMessage,
    utils::*,
};
use crate::entries::*;
use crate::entries::pub_enc_key::*;


/// Entry representing a received Manifest
#[hdk_entry(id = "DeliveryNotification", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotification {
    pub description: ParcelDescription,
    pub sender: AgentPubKey,
    pub sender_manifest_signature: Signature,
    pub sender_distribution_eh: EntryHash,
}

impl DeliveryNotification {

    pub fn from_direct(dm: ReceptionRequestMessage, from: AgentPubKey) -> Self {
        Self {
            description: dm.description,
            sender: from.clone(),
            sender_manifest_signature: dm.sender_description_signature,
            sender_distribution_eh: dm.sender_distribution_eh,
        }
    }


    pub fn try_from_pending(pending: PendingItem, from: AgentPubKey) -> ExternResult<Option<Self>> {
        /// Must be a pending ParcelDescription
        if pending.kind != PendingKind::Description {
            return error("Cannot convert PendingItem into DeliveryNotification because of wrong PendingItem kind");
        }
        /// Get my key
        let me = agent_info()?.agent_latest_pubkey;
        let recipient_key = get_enc_key(me.clone())?;
        debug!("try_from_pending() recipient_key: {:?}", recipient_key);
        /// Get sender's key
        let sender_key = get_enc_key(from.clone())?;
        debug!("try_from_pending() sender_key: {:?}", sender_key);
        /// Decrypt
        let maybe_description: Option<ParcelDescription> = pending.attempt_decrypt(sender_key, recipient_key);
        debug!("try_from_pending() maybe_description: {:?}", maybe_description);
        /// Into DeliveryNotification
        let notification = match maybe_description {
            None => return Ok(None),
            Some(description) => {
                Self {
                    description,
                    sender: from.clone(),
                    sender_distribution_eh: pending.sender_distribution_eh,
                    sender_manifest_signature: pending.sender_signature.clone()
                }
            },
        };
        /// Check signature
        let maybe_verified = verify_signature(from, pending.sender_signature, notification.description.clone());
        match maybe_verified {
            Err(err) => {
                let response_str = "Verifying PendingItem failed";
                debug!("{}: {}", response_str, err);
                return error(response_str);
            }
            Ok(false) => {
                let response_str = "Failed verifying PendingItem signature";
                debug!("{}", response_str);
                return error(response_str);
            }
            Ok(true) => debug!("Valid PendingItem signature"),
        }
        /// Done
        Ok(Some(notification))
    }
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
    /// Send confirmation to sender
    send_confirmation(
        notification.sender.clone(),
        notification.sender_distribution_eh.clone(),
        notification.clone(),
        signature,
    )?;

    /// Done
    Ok(())
}



///
pub(crate) fn send_item<T>(
    recipient: AgentPubKey,
    distribution_eh: EntryHash,
    item: T,
    signed_item: Signature,
) -> ExternResult<SendSuccessKind> {
    debug!("send_item() START - {:?}", recipient);


    // /// Shortcut to self
    // let me = agent_info()?.agent_latest_pubkey;
    // if recipient.clone() == me {
    //    debug!("request_reception() Self");
    //    let msg = MailMessage {
    //       outmail_eh: eh.clone(),
    //       mail: parcel_description.clone(),
    //       mail_signature: sender_description_signature.clone(),
    //    };
    //    let inmail = InMail::from_direct(msg, me.clone());
    //    debug!("request_reception() REMOTE CALLING...");
    //    let res = call_remote(
    //       me,
    //       zome_info()?.name,
    //       "commit_inmail".to_string().into(),
    //       None,
    //       inmail,
    //    )?;
    //    debug!("commit_inmail() END : {:?}", res);
    //    assert!(matches!(res, ZomeCallResponse::Ok { .. }));
    //    return Ok(SendSuccessKind::OK_SELF);
    // }


    /// Try sending directly to other Agent if Online
    let result = send_item_by_dm(recipient, distribution_eh, item, signed_item);
    if result.is_ok() {
        return Ok(SendSuccessKind::OK_DIRECT);
    } else {
        let err = result.err().unwrap();
        debug!("send_item() failed: {:?}", err);
    }

    debug!("send_item() - Creating PendingItem...");
    /// DM failed, send to DHT instead by creating a PendingMail
    /// Create and commit PendingMail with remote call to self
    let pending_item = PendingItem::from(
        item.clone(),
        distribution_eh.clone(),
        recipient.clone(),
    )?;
    let input = CommitPendingItemInput {
        item: pending_item,
        recipient: recipient.clone(),
    };
    debug!("send_item() - calling commit_pending_mail()");
    let response = call_remote(
        me,
        zome_info()?.name,
        "commit_pending_item".to_string().into(),
        None,
        input,
    )?;
    debug!("send_confirmation() - commit_pending_mail() response: {:?}", response);
    return match response {
        ZomeCallResponse::Ok(_) => Ok(SendSuccessKind::OK_PENDING),
        _ => error("call_remote() to commit_pending_item() failed")
    };
}



/// Attempt sending reception request via DM
fn send_item_by_dm<T>(
    recipient: AgentPubKey,
    distribution_eh: EntryHash,
    item: T,
    signed_item: Signature,
) -> ExternResult<()> {
    /// --  Send Mail
    debug!("send_confirmation_by_dm() to {}", recipient);
    /// Create DM
    let msg = ItemMessage {
        item,
        signed_item: signed_item,
        distribution_eh: distribution_eh,
    };
    /// Send DM
    let response_dm = send_dm(recipient, DeliveryProtocol::ReceptionRequest(msg))?;
    debug!("send_confirmation_by_dm() response_dm = {:?}", response_dm);
    /// Check Response
    if let DeliveryProtocol::Success(_) = response_dm {
        return Ok(());
    }
    return error(&format!("send_confirmation_by_dm() failed: {:?}", response_dm));
}