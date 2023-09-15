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
   /// Create ParcelSummary
   let parcel_size: u64 = match input.parcel_ref.kind_info.clone() {
      ParcelKind::AppEntry(_) => get_app_entry_size(input.parcel_ref.eh.clone())? as u64,
      ParcelKind::Manifest(_) => {
         let manifest: ParcelManifest = get_typed_from_eh(input.parcel_ref.eh.clone())?;
         manifest.chunks.len() as u64 * get_dna_properties().max_chunk_size as u64
         //parcel_name = manifest.name;
         //manifest.size
      }
   };
   let delivery_summary = DeliverySummary {
      distribution_strategy: input.strategy,
      parcel_description: ParcelDescription {
         name: input.parcel_name,
         size: parcel_size,
         reference: input.parcel_ref,
      },
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
