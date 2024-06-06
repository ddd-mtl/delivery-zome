use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::LinkTypes;
use zome_delivery_types::{ParcelReference};
use crate::*;


///
#[hdk_extern]
#[feature(zits_blocking)]
pub fn unpublish_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
   let Some(record) = get(pp_eh.clone(), GetOptions::network())? else {
      return error("No PublicParcel found at EntryHash");
   };
   /// Make sure its the correct entry type
   let _pr: ParcelReference = get_typed_from_record(record.clone())?;
   ///// Delete ParcelReference
   //let ah = delete_entry(record.action_address().to_owned())?;
   /// Delete Link
   let link_ah = unlink_public_parcel(pp_eh)?;
   /// FIXME: delete ParcelManifest
   // /// Emit Signal
   // let dg = DeliveryGossip {
   //   from: agent_info()?.agent_latest_pubkey,
   //   gossip: DeliveryGossipProtocol::PublicParcelUnpublished((pr_eh, record.signed_action.hashed.content.timestamp(), pr)),
   // };
   // let res = emit_gossip_signal(dg);
   // if let Err(err) = res {
   //   error!("Emit signal failed: {}", err);
   // }
   ///
   Ok(link_ah)
}


///
pub fn unlink_public_parcel(pp_eh: EntryHash) -> ExternResult<ActionHash> {
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
      if target == pp_eh {
         return delete_link(create_sah.hashed.hash);
      }
   }
   return error("Link to PublicParcel not found");
}
