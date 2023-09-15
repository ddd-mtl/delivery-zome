use hdk::prelude::*;
use zome_delivery_integrity::*;
use crate::public_parcels_path;


/// Called from PublicParcel post-commit since its created in a "relaxed" way
#[ignore]
#[hdk_extern]
pub fn link_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
   let path = public_parcels_path();
   path.ensure()?;
   let anchor_eh = path.path_entry_hash()?;
   let ah = create_link(anchor_eh, pp_eh.clone(), LinkTypes::PublicParcels, LinkTag::from(()))?;
   Ok(ah)
}