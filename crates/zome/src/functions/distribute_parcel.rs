use hdk::prelude::*;
use zome_delivery_types::*;
use crate::utils_parcel::*;
use crate::utils::*;


/// Zone Function
#[hdk_extern]
pub fn distribute_parcel(input: DistributeParcelInput) -> ExternResult<EntryHash> {
   debug!("distribute_parcel() START: {:?}", input);
   /// Remove duplicate recipients
   let mut recipients = input.recipients.clone();
   let set: HashSet<_> = recipients.drain(..).collect(); // dedup
   recipients.extend(set.into_iter());
   debug!("distribute_parcel() recipients: {}", recipients.len());
   /// Create ParcelSummary
   let parcel_summary = match input.parcel_kind {
      ParcelKind::AppEntry(app_type) => {
         ParcelSummary {
            size: get_app_entry_size(input.parcel_eh.clone())?,
            reference: ParcelReference::AppEntry((app_type, input.parcel_eh))
         }
      }
      ParcelKind::Manifest => {
         let manifest: ParcelManifest = get_typed_from_eh(input.parcel_eh.clone())?;
         ParcelSummary {
            size: manifest.size,
            reference: ParcelReference::Manifest(input.parcel_eh),
         }
      }
   };
   debug!("distribute_parcel() parcel_summary: {:?}", parcel_summary);
   /// Sign summary
   let summary_signature = sign(agent_info()?.agent_latest_pubkey, parcel_summary.clone())?;
   /// Create Distribution
   let distribution = Distribution {
      recipients,
      parcel_summary,
      summary_signature,
      strategy: input.strategy,
   };
   /// Commit Distribution
   let eh = hash_entry(distribution.clone())?;
   debug!("distribute_parcel() eh: {}", eh);
   let _hh = create_entry(distribution)?;
   /// Done
   Ok(eh)
}
