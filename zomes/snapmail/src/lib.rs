#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;

mod utils;
mod constants;
mod link_kind;
mod entry_kind;
mod path_kind;

mod dm;
mod dm_protocol;

mod signal_protocol;

mod callbacks;
mod handle;
mod mail;

mod file;

use hdk::prelude::*;

pub use dm::*;
pub use dm_protocol::*;
pub use utils::*;
pub use constants::*;
pub use link_kind::*;
pub use entry_kind::*;
pub use path_kind::*;
pub use signal_protocol::*;


//holochain_externs!();


/// -- Wrapped Common types -- ///

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZomeU32(pub u32);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZomeBool(bool);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZomeString(pub String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZomeRaw(Vec<u8>);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZomeHhVec(Vec<HeaderHash>);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZomeEhVec(Vec<EntryHash>);
