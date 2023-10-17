use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;


/// Return ehs of all ParcelManifest
#[hdk_extern]
pub fn get_all_private_manifests(_:()) -> ExternResult<Vec<(EntryHash, ParcelManifest)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ParcelManifest Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(DeliveryEntryTypes::PrivateManifest.try_into().unwrap());
   let records = query(query_args)?;
   debug!("private manifest found: {}", records.len());
   /// Convert records to ParcelManifests
   let pairs: Vec<(EntryHash, ParcelManifest)> = records.into_iter().map(|record| {
      let manifest: ParcelManifest = get_typed_from_record(record.clone()).unwrap();
      (record.action().entry_hash().unwrap().to_owned(), manifest)
   }).collect();
   debug!("pairs: {}", pairs.len());
   /// Done
   Ok(pairs)
}


/// Return ehs of all PublicManifest
#[hdk_extern]
pub fn get_all_local_public_manifests(_:()) -> ExternResult<Vec<(EntryHash, ParcelManifest)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ParcelManifest Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(DeliveryEntryTypes::PublicManifest.try_into().unwrap());
   let records = query(query_args)?;
   debug!("local public manifest found: {}", records.len());
   /// Convert records to ParcelManifests
   let pairs: Vec<(EntryHash, ParcelManifest)> = records.into_iter().map(|record| {
      let manifest: ParcelManifest = get_typed_from_record(record.clone()).unwrap();
      (record.action().entry_hash().unwrap().to_owned(), manifest)
   }).collect();
   debug!("pairs: {}", pairs.len());
   /// Done
   Ok(pairs)
}