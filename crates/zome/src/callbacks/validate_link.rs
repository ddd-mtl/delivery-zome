use hdk::prelude::*;

use strum::IntoEnumIterator;
use crate::link_kind::*;


/// Zome Callback
#[hdk_extern]
fn validate_create_link(candidat: ValidateCreateLinkData)
   -> ExternResult<ValidateLinkCallbackResult>
{
   let tag_str = String::from_utf8_lossy(&candidat.link_add.tag.0);
   trace!("*** `validate_create_link()` callback called: {}", tag_str);

   for link_kind in LinkKind::iter() {
      /// Try validating static link kind
      if tag_str == link_kind.as_ref() {
         return link_kind.validate_types(candidat, None);
      }
      /// Or try validating dynamic link kind
      let maybe_hash: ExternResult<AgentPubKey> = link_kind.unconcat_hash(&candidat.link_add.tag);
      //debug!("*** maybe_hash of {} = {:?}", link_kind.as_static(), maybe_hash);
      if let Ok(from) = maybe_hash {
         return link_kind.validate_types(candidat, Some(from));
      }
   }
   Ok(ValidateLinkCallbackResult::Invalid(format!("Unknown tag: {}", tag_str).into()))
}


/// Zome Callback
/// TODO: Should not be valide by default
#[hdk_extern]
fn validate_delete_link(_delete_link_submission: ValidateDeleteLinkData)
   -> ExternResult<ValidateLinkCallbackResult>
{
   trace!("*** validate_delete_link() callback called!");
   //let _delete_link = validate_delete_link.delete_link;
   // Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
   Ok(ValidateLinkCallbackResult::Valid)
}