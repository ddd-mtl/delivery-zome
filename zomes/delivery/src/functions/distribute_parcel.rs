use hdk::prelude::*;
use crate::{
   get_typed_from_eh,
   entries::*,
   utils_parcel::*,
   parcel::*,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeParcelInput {
   pub recipients: Vec<AgentPubKey>,
   pub strategy: DistributionStrategy,
   pub parcel_kind: ParcelKind,
   pub parcel_eh: EntryHash,
}

/// Zone Function
#[hdk_extern]
pub fn distribute_parcel(input: DistributeParcelInput) -> ExternResult<EntryHash> {
   debug!("distribute_parcel(): {}", input.parcel_eh);
   /// Remove duplicate recipients
   let mut recipients = input.recipients.clone();
   let set: HashSet<_> = recipients.drain(..).collect(); // dedup
   recipients.extend(set.into_iter());
   /// Create ParcelSummary
   let parcel_summary = match input.parcel_kind {
      ParcelKind::AppEntry(app_type) => {
         ParcelSummary {
            size: get_app_entry_size(parcel_eh)?,
            parcel: Parcel::AppEntry((app_type, parcel_eh))
         }
      }
      ParcelKind::Manifest => {
         let manifest: ParcelManifest = get_typed_from_eh(input.parcel)?;
         ParcelSummary {
            size: manifest.size,
            parcel: Parcel::Package(input.parcel_eh),
         }
      }
   };
   /// Sign description
   let me = agent_info()?.agent_latest_pubkey;
   let sender_description_signature = sign(me, desc)?;
   /// Create Distribution
   let distribution = Distribution {
      recipients,
      parcel_description: parcel_summary,
      strategy: input.strategy,
      sender_description_signature,
   };
   /// Commit Distribution
   let eh = hash_entry(distribution.clone())?;
   let _hh = create_entry(distribution)?;
   Ok(eh)
}
