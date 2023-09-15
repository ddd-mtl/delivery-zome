use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;


/// Create & Commit Distribution Entry.
/// It will try to send deliveryNotice during its post_commit().
#[hdk_extern]
pub fn distribute_parcel(input: DistributeParcelInput) -> ExternResult<ActionHash> {
   debug!("START: {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Remove duplicate recipients
   let mut recipients = input.recipients.clone();
   let set: HashSet<_> = recipients.drain(..).collect(); // dedup
   recipients.extend(set.into_iter());
   debug!("recipients: {}", recipients.len());
   /// Compute Parcel size
   let mut parcel_reference = input.parcel_reference.clone();
   if parcel_reference.description.size == 0 {
      parcel_reference.description.size = match input.parcel_reference.description.kind_info.clone() {
         ParcelKind::AppEntry(_) => get_app_entry_size(input.parcel_reference.eh.clone())? as u64,
         ParcelKind::Manifest(_) => {
            let manifest: ParcelManifest = get_typed_from_eh(input.parcel_reference.eh.clone())?;
            manifest.description.size
         }
      };
   }
   /// Create DeliverySummary
   let delivery_summary = DeliverySummary {
      distribution_strategy: input.strategy,
      parcel_reference
   };
   debug!("delivery_summary: {:?}", delivery_summary);
   /// Sign summary
   let summary_signature = sign(agent_info()?.agent_latest_pubkey, delivery_summary.clone())?;
   /// Create Distribution
   let distribution = Distribution {
      recipients,
      delivery_summary,
      summary_signature,
   };
   /// Commit Distribution
   let eh = hash_entry(distribution.clone())?;
   debug!("eh: {}", eh);
   let ah = create_entry(DeliveryEntry::Distribution(distribution))?;
   //let ah = create_entry_relaxed(DeliveryEntry::Distribution(distribution))?;
   /// Done
   Ok(ah)
}
