#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


use hdi::prelude::*;


#[hdk_link_types]
pub enum LinkTypes {
   Default
}

#[hdk_entry_types]
#[unit_enum(SecretEntryTypes)]
pub enum SecretEntry {
   #[entry_type(required_validations = 2, visibility = "private")]
   Secret(Secret),
}

/// Entry representing a secret message
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Secret {
   pub value: String,
}


