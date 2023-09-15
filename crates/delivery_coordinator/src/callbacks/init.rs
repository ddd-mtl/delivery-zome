use hdk::prelude::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

//#[hdk_extern]
fn init_caps(_: ()) -> ExternResult<()> {
   let mut functions = BTreeSet::new();
   functions.insert((zome_info()?.name, REMOTE_ENDPOINT.into()));
   //functions.insert((zome_info()?.name, "get_enc_key".into()));
   create_cap_grant(
      CapGrantEntry {
         tag: "".into(),
         access: ().into(), // empty access converts to unrestricted
         functions: hdk::prelude::GrantedFunctions::Listed(functions),
      }
   )?;
   Ok(())
}


/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** zDelivery.init() callback START");
   /// Set Global Anchors
   Path::from(DIRECTORY_PATH).typed(LinkTypes::Members)?.ensure()?;
   /// Setup initial capabilities
   init_caps(())?;
   ///
   let res = validate_properties()?;
   let InitCallbackResult::Pass = res
      else { return Ok(res); };
   /// Create public encryption key and broadcast it
   create_enc_key()?;
   /// Done
   debug!("*** zDelivery.init() callback DONE");
   Ok(InitCallbackResult::Pass)
}


///
fn validate_properties() -> ExternResult<InitCallbackResult> {
   let maybe_properties = get_properties();
   //debug!("maybe_place_properties = {:?}", maybe_place_properties);
   if let Err(e) = &maybe_properties {
      let err_msg = format!("Failed parsing DNA properties: {:?}", e);
      error!(err_msg);
      return Ok(InitCallbackResult::Fail(err_msg));
   }
   let dna_properties = maybe_properties.unwrap();
   debug!("*** validate dna properties: {:?}", dna_properties);
   if dna_properties.max_chunk_size == 0 {
      let err_msg = format!("Invalid DNA property \"max_chunk_size\" must be bigger than 0 . value: {}", dna_properties.max_chunk_size);
      return Ok(InitCallbackResult::Fail(err_msg));
   }
   if dna_properties.max_chunk_size as u64 > dna_properties.max_parcel_size {
      let err_msg = format!("Invalid DNA property \"max_file_size\" must be bigger than \"max_chunk_size\" . values: {} | {} ", dna_properties.max_parcel_size, dna_properties.max_chunk_size);
      return Ok(InitCallbackResult::Fail(err_msg));
   }
   if dna_properties.max_parcel_name_length == 0 {
      let err_msg = format!("Invalid DNA property \"max_parcel_name_length\" must be bigger than 0 . value: {}", dna_properties.max_parcel_name_length);
      return Ok(InitCallbackResult::Fail(err_msg));
   }
   if dna_properties.min_parcel_name_length as u32 > dna_properties.max_parcel_name_length {
      let err_msg = format!("Invalid DNA property \"min_parcel_name_length\" must be bigger than \"max_parcel_name_length\" . values: {} | {} ", dna_properties.min_parcel_name_length, dna_properties.max_parcel_name_length);
      return Ok(InitCallbackResult::Fail(err_msg));
   }
   Ok(InitCallbackResult::Pass)
}