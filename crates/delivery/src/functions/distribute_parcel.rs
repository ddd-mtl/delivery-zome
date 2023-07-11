use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

use crate::utils_parcel::*;


/// Zone Function
#[hdk_extern]
pub fn distribute_parcel(input: DistributeParcelInput) -> ExternResult<EntryHash> {
   debug!("distribute_parcel() START: {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Remove duplicate recipients
   let mut recipients = input.recipients.clone();
   let set: HashSet<_> = recipients.drain(..).collect(); // dedup
   recipients.extend(set.into_iter());
   debug!("distribute_parcel() recipients: {}", recipients.len());
   /// Create ParcelSummary
   let size = match input.parcel_ref.clone() {
      ParcelReference::AppEntry(_, _, eh) => get_app_entry_size(eh)?,
      ParcelReference::Manifest(eh) => {
         let manifest: ParcelManifest = get_typed_from_eh(eh.clone())?;
         manifest.size
      }
   };
   let delivery_summary = DeliverySummary {
      parcel_size: size,
      distribution_strategy: input.strategy,
      parcel_reference: input.parcel_ref,
   };
   debug!("distribute_parcel() delivery_summary: {:?}", delivery_summary);
   /// Sign summary
   let summary_signature = sign(agent_info()?.agent_latest_pubkey, delivery_summary.clone())?;
   /// Create Distribution
   let distribution = Distribution {
      recipients,
      delivery_summary: delivery_summary,
      summary_signature,
   };
   /// Commit Distribution
   let eh = hash_entry(distribution.clone())?;
   debug!("distribute_parcel() eh: {}", eh);
   let _hh = create_entry_relaxed(distribution)?;
   /// Done
   Ok(eh)
}
