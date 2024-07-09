use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::{ParcelReference};


///
#[hdk_extern]
#[feature(zits_blocking)]
pub fn unpublish_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
   let Some(record) = get(pp_eh.clone(), GetOptions::network())? else {
      return error("No PublicParcel found at EntryHash");
   };
   /// Make sure its the correct entry type
   let _pr: ParcelReference = get_typed_from_record(record.clone())?;
   /// Delete ParcelReference
   let ah = delete_entry_relaxed(record.action_address().to_owned())?;
   ///
   Ok(ah)
}
