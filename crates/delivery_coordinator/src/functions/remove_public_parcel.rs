use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::LinkTypes;
use zome_delivery_types::ParcelReference;
use crate::SignalProtocol;
use crate::public_parcels_path;


///
#[hdk_extern]
pub fn remove_public_parcel(pr_eh: EntryHash) -> ExternResult<ActionHash> {
   let Some(record) = get(pr_eh.clone(), GetOptions::network())? else {
      return error("No entry found at EntryHash");
   };
   /// Make sure its the correct entry type
   let pr: ParcelReference = get_typed_from_record(record.clone())?;
   /// Delete ParcelReference
   let ah = delete_entry(record.action_address().to_owned())?;
   /// Delete Link
   let _link_ah = unlink_public_parcel(pr_eh.clone())?;
   /// FIXME: delete ParcelManifest
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::RemovedPublicParcel((pr_eh, record.signed_action.hashed.content.timestamp(), pr, agent_info()?.agent_latest_pubkey)));
   if let Err(err) = res {
     error!("Emit signal failed: {}", err);
   }
   ///
   Ok(ah)
}


///
pub fn unlink_public_parcel(pr_eh: EntryHash) -> ExternResult<ActionHash> {
   let path = public_parcels_path();
   path.ensure()?;
   let anchor_eh = path.path_entry_hash()?;
   let links = get_link_details(anchor_eh, LinkTypes::PublicParcels, None, GetOptions::network())?;
   for (create_sah, maybe_deletes) in links.into_inner() {
      if !maybe_deletes.is_empty() {
         continue;
      }
      let Action::CreateLink(create) = create_sah.hashed.content else { panic!("get_link_details() should return a CreateLink Action")};
      let target = EntryHash::try_from(create.target_address).unwrap();
      if target == pr_eh {
         return delete_link(create_sah.hashed.hash);
      }
   }
   return error("Link to PublicParcel not found");
}
