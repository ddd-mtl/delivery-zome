use hdk::prelude::*;
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
