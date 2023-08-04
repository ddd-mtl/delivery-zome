use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;


/// Create & Commit Distribution Entry.
/// It will try to send deliveryNotice during its post_commit().
#[hdk_extern]
pub fn distribute_parcel(input: DistributeParcelInput) -> ExternResult<EntryHash> {
   debug!("START: {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Remove duplicate recipients
   let mut recipients = input.recipients.clone();
   let set: HashSet<_> = recipients.drain(..).collect(); // dedup
   recipients.extend(set.into_iter());
   debug!("recipients: {}", recipients.len());
   /// Create ParcelSummary
   let size = match input.parcel_ref.clone() {
      ParcelReference::AppEntry(eref) => get_app_entry_size(eref.eh)?,
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
   debug!("delivery_summary: {:?}", delivery_summary);
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
   debug!("eh: {}", eh);
   let _hh = create_entry(DeliveryEntry::Distribution(distribution))?;
   //let _hh = create_entry_relaxed(DeliveryEntry::Distribution(distribution))?;
   /// Done
   Ok(eh)
}
