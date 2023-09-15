use hdi::prelude::*;

/// Dna properties
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryProperties {
   pub max_chunk_size: u32,
   pub max_parcel_size: u64,
   pub max_parcel_name_length: u32,
   pub min_parcel_name_length: u16,
}


/// Return the DNA properties
pub fn get_properties() -> ExternResult<DeliveryProperties> {
   //debug!("*** get_properties() called");
   let dna_info = dna_info()?;
   let props = dna_info.modifiers.properties;
   //debug!("props = {:?}", props);
   let maybe_properties: Result<DeliveryProperties, <DeliveryProperties as TryFrom<SerializedBytes>>::Error> = props.try_into();
   if let Err(e) = maybe_properties {
      debug!("deserializing properties failed: {:?}", e);
      panic!("Should deserialize dna properties");
   }
   Ok(maybe_properties.unwrap())
}


/// Helper for crate use
pub fn get_dna_properties() -> DeliveryProperties {
   return get_properties().unwrap();
}