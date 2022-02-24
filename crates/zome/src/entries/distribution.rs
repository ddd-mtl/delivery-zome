use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use crate::send_item::*;
use crate::functions::*;
use crate::utils::*;

impl ZomeEntry for Distribution {
    ///
    fn validate(&self, _maybe_package: Option<ValidationPackage>) -> ExternResult<ValidateCallbackResult> {
        if self.recipients.is_empty() {
            return invalid("Need at least one recipient");
        }

        /// FIXME: validate parcel ; make sure Parcel entry has been committed
        //validate_parcel(input.parcel_description)?;

        Ok(ValidateCallbackResult::Valid)
    }


    ///
    fn post_commit(&self, distribution_eh: &EntryHash) -> ExternResult<()> {
        debug!("post_commit_distribution() {:?}", distribution_eh);

        /// FIXME match distribution.strategy

        /// Create DeliveryNotice
        let notice = DeliveryNotice {
            distribution_eh: distribution_eh.clone(),
            sender: agent_info()?.agent_latest_pubkey,
            parcel_summary: self.parcel_summary.clone(),
            sender_summary_signature: self.summary_signature.clone(),
        };
        /// Sign notice
        //let signature = sign(agent_info()?.agent_latest_pubkey, notice.clone())?;
        /// Send to each recipient
        for recipient in self.recipients.clone() {
            /// Create PendingItem
            let pending_item = pack_notice(
                notice.clone(),
                recipient.clone(),
            )?;
            /// Send it to recipient
            let res = send_item(
                recipient,
                //distribution_eh.clone(),
                pending_item,
               // signature.clone(),
            );
            match res {
                Ok(_) => {},
                Err(e) => {
                    /// FIXME: accumulate failed recipients to final error return value
                    debug!("send_reception_request() failed: {}", e);
                }
            }
        }
        /// Done
        Ok(())
    }
}
