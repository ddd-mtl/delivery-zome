use hdk::prelude::*;
use zome_utils::call_self;
use zome_delivery_types::*;
use crate::*;



///
pub fn post_commit_Distribution(entry: Entry, distribution_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_distribution() {:?}", distribution_eh);
    let distribution = Distribution::try_from(entry)?;
    /// Create DeliveryNotice
    let notice = DeliveryNotice {
        distribution_eh: distribution_eh.clone(),
        sender: agent_info()?.agent_latest_pubkey,
        summary: distribution.delivery_summary.clone(),
        sender_summary_signature: distribution.summary_signature.clone(),
    };
    /// Send to each recipient
    for recipient in distribution.recipients.clone() {
        /// Create PendingItem
        let pending_item = pack_notice(
            notice.clone(),
            recipient.clone(),
        )?;
        /// Send it to recipient
        let res = send_item(
            recipient.clone(),
            pending_item.clone(),
            distribution.delivery_summary.distribution_strategy.clone(),
            // signature.clone(),
        );
        /// FIXME: accumulate failed recipients to final error return value
        if let Err(e) = res {
            warn!("send_item() during Distribution::post_commit() failed: {}", e);
            continue;
        }
        /// If direct-send succeeded, create NoticeReceived Entry
        let response = res.unwrap();
        if let SendSuccessKind::OK_DIRECT(signature) = response {
            let valid = verify_signature(recipient.clone(), signature.clone(), notice.summary.clone())?;
            if !valid {
                warn!("Recipient failed to sign Notice. Suspicious behavior.");
                continue;
            }
            let ack = NoticeReceived {
                distribution_eh: notice.distribution_eh.clone(),
                recipient: recipient.clone(),
                recipient_summary_signature: signature.clone(),
                date_of_reception: sys_time()?,
            };
            let _ = call_self("commit_NoticeReceived", ack)?;
        }
    }
    /// Done
    Ok(())
}
