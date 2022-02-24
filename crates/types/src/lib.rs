#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use] extern crate enum_ordinalize;

//----------------------------------------------------------------------------------------

mod entry_kind;
mod entries;
mod parcel;
mod zfn_inputs;

pub use entries::*;
pub use zfn_inputs::*;
pub use parcel::*;
pub use entry_kind::*;