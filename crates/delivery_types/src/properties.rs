use hdi::prelude::*;

/// Dna properties
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryProperties {
   pub max_parcel_size: u64,
   pub max_chunk_size: u32,
   pub max_parcel_name_length: u32,
   pub min_parcel_name_length: u16,
}

impl DeliveryProperties {
   pub fn new(max_chunk_size: u32, max_parcel_size: u64, min_parcel_name_length: u16, max_parcel_name_length: u32) -> Self {
      Self { max_chunk_size, max_parcel_size, max_parcel_name_length, min_parcel_name_length }
   }
}

/// Return the DNA properties
pub fn get_properties() -> ExternResult<DeliveryProperties> {
   //debug!("*** get_properties() called");
   let dna_info = dna_info()?;
   let props = dna_info.modifiers.properties;
   //debug!("props = {:?}", props);
   let maybe_properties: Result<DeliveryProperties, <DeliveryProperties as TryFrom<SerializedBytes>>::Error> = props.try_into();
   if let Err(e) = maybe_properties {
      debug!("Deserializing DeliveryZome properties failed: {:?}", e);
      return Err(wasm_error!("Deserializing DeliveryZome properties failed: {:?}", e));
   }
   Ok(maybe_properties.unwrap())
}



impl DeliveryProperties {
   pub fn validate(&self) -> ExternResult<ValidateCallbackResult> {
      if self.max_parcel_name_length == 0 {
         return Ok(ValidateCallbackResult::Invalid("DNA Property \"max_parcel_name_length\" must be > 0".to_string()));
      }
      if self.max_parcel_name_length < self.min_parcel_name_length as u32 {
         return Ok(ValidateCallbackResult::Invalid("DNA Property \"max_parcel_name_length\" must be bigger than \"min_parcel_name_length\"".to_string()));
      }
      if self.max_chunk_size == 0 {
         return Ok(ValidateCallbackResult::Invalid("DNA Property \"max_chunk_size\" must be > 0".to_string()));
      }
      if self.max_parcel_size == 0 {
         return Ok(ValidateCallbackResult::Invalid("DNA Property \"max_parcel_size\" must be > 0".to_string()));
      }
      if self.max_parcel_size < self.max_chunk_size as u64 {
         return Ok(ValidateCallbackResult::Invalid("DNA Property \"max_parcel_size\" must be bigger than \"max_chunk_size\"".to_string()));
      }
      ///
      Ok(ValidateCallbackResult::Valid)
   }
}


#[cfg(test)]
mod tests {
   use super::*;

   fn assert_invalid(properties: DeliveryProperties) {
      let result = properties.validate().expect("validation should not fail");
      match result {
         ValidateCallbackResult::Invalid(_message) => (),
         other => panic!("expected invalid validation result, got {:?}", other),
      }
   }

   fn assert_valid(properties: DeliveryProperties) {
      let result = properties.validate().expect("validation should not fail");
      match result {
         ValidateCallbackResult::Valid => (),
         other => panic!("expected valid validation result, got {:?}", other),
      }
   }

   #[test]
   fn valid_properties() {
      assert_valid(DeliveryProperties::new(1024, 1024 * 1024, 2, 10));
      assert_valid(DeliveryProperties::new(1024, 1024, 2, 10));
      assert_valid(DeliveryProperties::new(1024, 1024 * 1024, 10, 10));
   }

   #[test]
   fn invalid_properties() {
      assert_invalid(DeliveryProperties::new(0, 1024 * 1024, 2, 10));
      assert_invalid(DeliveryProperties::new(1024, 0, 2, 10));
      assert_invalid(DeliveryProperties::new(1024, 1024 * 1024, 2, 0));
      assert_invalid(DeliveryProperties::new(1024, 1024 * 1024, 10, 2));
      assert_invalid(DeliveryProperties::new(1024 * 1024, 1024, 10, 2));
   }
}
