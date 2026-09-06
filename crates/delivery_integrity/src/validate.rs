use hdi::prelude::*;
use crate::validate_app_entry::validate_app_entry;

///
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
   //debug!("*** DeliveryIntegrityZome.validate() op = {:?}", op);
   match op {
      Op::CreateRecord ( _ ) => Ok(ValidateCallbackResult::Valid),
      Op::CreateEntry(createEntry) => {
         let creation_action = createEntry.action.hashed.into_inner().0;
         let maybe_entry_type = creation_action.entry_type().cloned();
         return validate_entry(creation_action, createEntry.entry, maybe_entry_type.as_ref());
      },
      Op::CreateLink(_reg_create_link) => {
         // FIXME return validate_create_link(reg_create_link.create_link);
         Ok(ValidateCallbackResult::Valid)
      },
      Op::DeleteLink (_)=> Ok(ValidateCallbackResult::Valid),
      Op::Update { .. } => Ok(ValidateCallbackResult::Valid),
      Op::Delete { .. } => Ok(ValidateCallbackResult::Valid),
      Op::AgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
   }
}


///
pub fn validate_entry(creation_action: Action, entry: Entry, maybe_entry_type: Option<&EntryType>) -> ExternResult<ValidateCallbackResult> {
   /// Determine where to dispatch according to base
   let result = match entry.clone() {
      Entry::CounterSign(_data, _bytes) => Ok(ValidateCallbackResult::Invalid("CounterSign not allowed".into())),
      Entry::Agent(_agent_key) => Ok(ValidateCallbackResult::Valid),
      Entry::CapClaim(_claim) => Ok(ValidateCallbackResult::Valid),
      Entry::CapGrant(_grant) => Ok(ValidateCallbackResult::Valid),
      Entry::App(_entry_bytes) => {
         let EntryType::App(app_entry_def) = maybe_entry_type.unwrap()
            else { unreachable!() };
         let entry_def_index = validate_app_entry(creation_action, app_entry_def.entry_index(), entry);
         entry_def_index
      },
   };
   /// Done
   //debug!("*** validate_entry() result = {:?}", result);
   result
}
