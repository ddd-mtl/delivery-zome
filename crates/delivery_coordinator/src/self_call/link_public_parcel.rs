use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use crate::public_parcels_path;


/// Called from PublicParcel post-commit since its created in a "relaxed" way
#[ignore(zits)]
#[hdk_extern]
pub fn link_public_parcel(pr_eh: EntryHash) -> ExternResult<ActionHash> {
   let path = public_parcels_path();
   path.ensure()?;
   let anchor_eh = path.path_entry_hash()?;
   debug!("link_public_parcel() {} | {}", pr_eh, anchor_eh);
   let ah = create_link(anchor_eh, pr_eh.clone(), LinkTypes::PublicParcels, LinkTag::from(()))?;
   Ok(ah)
}




/// Called from PublicParcel post-commit
#[ignore(zits)]
#[hdk_extern]
pub fn unlink_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
   let path = public_parcels_path();
   path.ensure()?;
   let anchor_eh = path.path_entry_hash()?;
   let links = get_link_details(anchor_eh, LinkTypes::PublicParcels, None, GetOptions::network())?;
   for (create_sah, maybe_deletes) in links.into_inner() {
      if !maybe_deletes.is_empty() {
         continue;
      }
      let Action::CreateLink(create) = create_sah.hashed.content
        else { return zome_error!("get_link_details() should return a CreateLink Action")};
      let target = EntryHash::try_from(create.target_address).unwrap();
      if target == pp_eh {
         return delete_link(create_sah.hashed.hash);
      }
   }
   return zome_error!("Link to PublicParcel not found");
}
