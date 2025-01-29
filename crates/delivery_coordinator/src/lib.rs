mod callbacks;
mod functions;
mod self_call;

mod dm_protocol;
mod receive;
mod send_dm;
mod send_item;
mod utils_parcel;
mod unpack_item;
mod pack_item;

pub use dm_protocol::*;
pub use receive::*;
pub use send_dm::*;
pub use send_item::*;
pub use utils_parcel::*;
pub use functions::*;
pub use pack_item::*;

//pub use callbacks::*;
pub use self_call::*;


use hdk::prelude::*;

#[hdk_extern]
fn get_zome_info(_:()) -> ExternResult<ZomeInfo> {
  return zome_info();
}


#[hdk_extern]
fn get_dna_info(_:()) -> ExternResult<DnaInfo> {
  return dna_info();
}


#[hdk_extern]
fn get_record_author(dh: AnyDhtHash) -> ExternResult<AgentPubKey> {
  return zome_utils::get_author(dh);
}
