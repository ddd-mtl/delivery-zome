use hdk::prelude::*;

use crate::{
   mail::get_outacks,
   utils::*,
};
use crate::entries::InMail;
use crate::utils_parcel::get_confirmations;


/// Zome function
#[hdk_extern]
#[snapmail_api]
pub fn has_ack_been_delivered(inmail_hh: HeaderHash) -> ExternResult<bool> {
   /// Make sure its an inmail
   let inmail_eh = hh_to_eh(inmail_hh.clone())?;
   let _ = get_typed_from_eh::<InMail>(inmail_eh)?;
   /// Get inmail's outack
   let inacks = get_outacks(Some(inmail_hh))?;
   if inacks.is_empty() {
      return Ok(false)
   }
   let inack_eh = hash_entry(inacks[0].clone())?;
   /// Check for OutAck's confirmation
   let confirmations = get_confirmations(inack_eh)?;
   return Ok(!confirmations.is_empty());
}
