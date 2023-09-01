use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;


/// Return ehs of all ParcelManifest for type FILE_TYPE_NAME
#[hdk_extern]
pub fn get_all_local_parcels(_:()) -> ExternResult<Vec<(EntryHash, ParcelManifest)>> {
   /// Get all Create ParcelManifest Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(DeliveryEntryTypes::ParcelManifest.try_into().unwrap());
   let records = query(query_args)?;
   /// Convert records to ParcelManifests
   let pairs = records.into_iter().map(|record| {
      let manifest: ParcelManifest = get_typed_from_record(record.clone()).unwrap();
      (record.action().entry_hash().unwrap().to_owned(), manifest)
   }).collect();
   /// Done
   Ok(pairs)
}