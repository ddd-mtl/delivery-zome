use hdk::prelude::*;

use crate::{
    self::*,
    get_typed_from_eh,
    entries::*,
    send_item::*,
    parcel::*,
};



#[allow(non_camel_case_types)]
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

///
pub fn validate_distribution(input: Distribution) -> Result<(), String> {
    if recipients.is_empty() {
        return Err("Missing a recipient or parcel".to_owned());
    }

    /// FIXME: validate parcel
    //validate_parcel(input.parcel_description)?;

    Ok(())
}

///
pub fn post_commit_Distribution(distribution_eh: &EntryHash, distribution: Distribution) -> ExternResult<()> {
    debug!("post_commit_distribution() {:?}", distribution_eh);

    /// FIXME match distribution.strategy

    /// Send to each recipient
    for recipient in distribution.recipients {
        /// Create PendingItem
        let pending_item = PendingItem::from_description(
            distribution.parcel_summary.clone(),
            distribution_eh.clone(),
            recipient.clone(),
        )?;
        /// Send it to recipient
        let res = send_item(
            recipient,
            distribution_eh.clone(),
            pending_item,
            distribution.summary_signature.clone());
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