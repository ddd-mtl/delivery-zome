use hdk::prelude::*;

use crate::{
    //self::*,
    //utils::get_typed_from_eh,
    entries::*,
    send_item::*,
    parcel::*,
};



#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DistributionStrategy {
    /// DM first, DHT otherwise
    NORMAL,
    /// Publish to DHT unencrypted,
    PUBLIC,
    /// Encrypt to recipients on DHT
    DHT_ONLY,
    /// Only via DM
    DM_ONLY,
}

/// Entry representing a request to send a Parcel to one or multiple recipients
#[hdk_entry(id = "Distribution", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Distribution {
    pub recipients: Vec<AgentPubKey>,
    pub parcel_summary: ParcelSummary,
    pub strategy: DistributionStrategy,
    pub summary_signature: Signature,
    //pub can_share_between_recipients: bool, // Make recipient list "public" to recipients
}

impl Distribution {
    ///
    pub fn validate(&self) -> Result<(), String> {
        if self.recipients.is_empty() {
            return Err("Missing a recipient or parcel".to_owned());
        }

        /// FIXME: validate parcel ; make sure Parcel entry has been committed
        //validate_parcel(input.parcel_description)?;

        Ok(())
    }

    ///
    pub fn post_commit(distribution_eh: &EntryHash, distribution: Self) -> ExternResult<()> {
        debug!("post_commit_distribution() {:?}", distribution_eh);

        /// FIXME match distribution.strategy

        /// Create DeliveryNotice
        let notice = DeliveryNotice {
            distribution_eh: distribution_eh.clone(),
            sender: agent_info()?.agent_latest_pubkey,
            parcel_summary: distribution.parcel_summary,
            sender_summary_signature: distribution.summary_signature,
        };
        /// Sign notice
        let signature = sign(agent_info()?.agent_latest_pubkey, notice.clone())?;
        /// Send to each recipient
        for recipient in distribution.recipients {
            /// Create PendingItem
            let pending_item = PendingItem::from_notice(
                notice.clone(),
                recipient.clone(),
            )?;
            /// Send it to recipient
            let res = send_item(
                recipient,
                distribution_eh.clone(),
                pending_item,
                signature.clone(),
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
