use hdk::prelude::*;
use zome_utils::{call_self, zome_error};
use zome_delivery_types::*;
use crate::*;



///
pub fn post_commit_Distribution(sah: &SignedActionHashed, entry: Entry, _distribution_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_distribution() {:?}", sah.action_address());
    let distribution = Distribution::try_from(entry)?;
    /// Create DeliveryNotice
    let notice = DeliveryNotice {
        distribution_ah: sah.action_address().to_owned(),
        sender: agent_info()?.agent_latest_pubkey,
        summary: distribution.delivery_summary.clone(),
        sender_summary_signature: distribution.summary_signature.clone(),
    };
    /// Send Notice to each recipient
    for recipient in distribution.recipients.clone() {
        /// FIXME: accumulate failed recipients to final error return value
        let _ = send_notice(notice.clone(), recipient, distribution.delivery_summary.distribution_strategy.clone());
    }
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::NewDistribution((sah.action_address().to_owned(), distribution, sah.hashed.content.timestamp())));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    }
    /// Done
    Ok(())
}


///
fn send_notice(notice: DeliveryNotice, recipient: AgentPubKey, distribution_strategy: DistributionStrategy) -> ExternResult<()> {
    debug!("send_notice() for: {}", notice.summary.parcel_name);
    /// Create PendingItem
    let pending_item= pack_notice(
        notice.clone(),
        recipient.clone(),
    )?;
    /// Send it to recipient
    let res = send_item(
        recipient.clone(),
        pending_item,
        distribution_strategy,
    );
    if let Err(e) = res {
        debug!("send_item() failed: {}", e);
        return zome_error!("send_item() failed: {}", e);
    }
    /// If direct-send succeeded, create NoticeAck Entry
    let response = res.unwrap();
    if let SendSuccessKind::OK_DIRECT(signature) = response {
        let valid = verify_signature(recipient.clone(), signature.clone(), notice.summary.clone())?;
        if !valid {
            warn!("Recipient failed to sign Notice. Suspicious behavior.");
            return zome_error!("Recipient failed to sign Notice. Suspicious behavior.");
        }
        let ack = NoticeAck {
            distribution_ah: notice.distribution_ah.clone(),
            recipient: recipient.clone(),
            recipient_summary_signature: signature.clone(),
        };
        let _ = call_self("commit_NoticeAck", ack)?;
    }
    Ok(())
}