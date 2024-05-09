use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::ParcelReference;
//use crate::public_parcels_path;


// ///
// #[hdk_extern]
// pub fn unlink_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
//    let path = public_parcels_path();
//    path.ensure()?;
//    let anchor_eh = path.path_entry_hash()?;
//    let ah = create_link(anchor_eh, pp_eh.clone(), LinkTypes::PublicParcels, LinkTag::from(()))?;
//    Ok(ah)
// }


///
#[hdk_extern]
pub fn remove_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
   let Some(record) = get(pp_eh, GetOptions::network())? else {
      return error("No entry found at EntryHash");
   };
   /// Make sure its the correct entry type
   let _: ParcelReference = get_typed_from_record(record.clone())?;
   /// Delete ParcelReference
   let ah = delete_entry(record.action_address().to_owned())?;
   /// FIXME: delete ParcelManifest
   ///
   Ok(ah)
}
