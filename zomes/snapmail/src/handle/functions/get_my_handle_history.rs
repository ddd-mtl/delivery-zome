use hdk3::prelude::*;

/*
use hdk::{
    holochain_persistence_api::{
        cas::content::Address
    },
};
*/

use crate::{
    handle::Handle,
};

/// Zome function
#[hdk_extern]
pub fn get_my_handle_history(initial_handle_address: HeaderHash) -> Vec<String> {

    let history_result = get_details(&initial_handle_address, GetOptions);
    if let Err(_e) = history_result {
        debug!("get_entry_history() failed").ok();
        return Vec::new();
    }
    let maybe_history = history_result.unwrap();
    if maybe_history.is_none() {
        debug!("No history found").ok();
        return Vec::new();
    }
    let history = maybe_history.unwrap();
    debug!("History length: {}", history.items.len()).ok();
    debug!("History crud_links length: {}", history.crud_links.len()).ok();

    let mut handle_list = Vec::new();

    for item in history.items {
        let handle_entry = item.entry.expect("should have entry");
        debug!("History headers length: {}", item.headers.len()).ok();
        debug!("item crud status: {:?}", item.meta.unwrap().crud_status).ok();
        let handle = crate::into_typed::<Handle>(handle_entry).expect("Should be Handle");
        handle_list.push(handle.name);
    }
    debug!("handle_list = {}", handle_list.len()).ok();
    handle_list
}