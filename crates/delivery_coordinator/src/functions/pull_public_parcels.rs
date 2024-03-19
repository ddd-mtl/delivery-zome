use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
/// Get All public Parcels
#[hdk_extern]
pub fn pull_public_parcels(_:()) -> ExternResult<Vec<(ParcelReference, Timestamp, AgentPubKey)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let anchor = public_parcels_path().path_entry_hash()?;
   let pps: Vec<(ParcelReference, Timestamp, AgentPubKey)> = get_typed_from_links::<ParcelReference>(link_input(anchor, LinkTypes::PublicParcels, None))
      .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
      .into_iter()
      .map(|(pp, link)| (pp, link.timestamp, link.author)).collect();
   debug!(" pps count: {}", pps.len());
   /// Done
   Ok(pps)
}