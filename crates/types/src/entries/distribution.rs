use hdk::prelude::*;

use crate::parcel::*;

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
