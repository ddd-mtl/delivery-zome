use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
/// Get All public Parcels
#[hdk_extern]
pub fn pull_public_parcels(_:()) -> ExternResult<Vec<ParcelReference>> {
   let anchor = public_parcels_path().path_entry_hash()?;
   let pps = get_typed_from_links::<ParcelReference>(anchor, LinkTypes::PublicParcels, None)
      .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
      .into_iter()
      .map(|(pp, _link)| pp).collect();
   Ok(pps)
}