use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;

use crate::{emit_self_signals, entry_signal, entry_signal_ah};


#[hdk_extern]
pub fn query_all(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_Distribution(())?;
   query_all_NoticeAck(())?;
   query_all_ReplyAck(())?;
   query_all_ReceptionAck(())?;
   query_all_DeliveryNotice(())?;
   query_all_NoticeReply(())?;
   query_all_ReceptionProof(())?;
   query_all_private_manifests(())?;
   query_all_public_manifests(())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_Distribution(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Emit System Signal
   let tuples = get_all_local(DeliveryEntryTypes::Distribution.try_into().unwrap())?;
   let signals = tuples.into_iter()
     .map(|(ah, create, entry)| entry_signal_ah(EntryStateChange::None, &create, entry2Kind(entry, DeliveryEntryTypes::Distribution).unwrap(), ah))
     .collect();
   emit_self_signals(signals)?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_DeliveryNotice(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<DeliveryNotice>(DeliveryEntryTypes::DeliveryNotice.try_into().unwrap())?;
   /// Done
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_NoticeAck(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<NoticeAck>(DeliveryEntryTypes::NoticeAck.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_NoticeReply(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<NoticeReply>(DeliveryEntryTypes::NoticeReply.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_ReplyAck(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ReplyAck>(DeliveryEntryTypes::ReplyAck.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_ReceptionProof(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ReceptionProof>(DeliveryEntryTypes::ReceptionProof.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_ReceptionAck(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ReceptionAck>(DeliveryEntryTypes::ReceptionAck.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_private_manifests(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ParcelManifest>(DeliveryEntryTypes::PrivateManifest.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_public_manifests(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ParcelManifest>(DeliveryEntryTypes::PublicManifest.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_public_chunks(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ParcelChunk>(DeliveryEntryTypes::PublicChunk.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_private_chunks(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<ParcelChunk>(DeliveryEntryTypes::PrivateChunk.try_into().unwrap())?;
   Ok(())
}


///
fn query_all_typed<R: TryFrom<Entry>>(entry_type: EntryType) -> ExternResult<()> {
   let EntryType::App(app_entry_def) = entry_type.clone()
     else { return error("Must be an App Entry type") };
   /// Emit System Signal
   let delivery_entry_type = entry_index_to_variant(app_entry_def.entry_index)?;
   let tuples = get_all_local(entry_type)?;
   let signals = tuples.into_iter()
     .map(|(_ah, create, entry)| entry_signal(EntryStateChange::None, &create, entry2Kind(entry, delivery_entry_type).unwrap()))
     .collect();
   emit_self_signals(signals)?;
   Ok(())
}


/// Return vec of typed entries of given entry type found in local source chain
fn get_all_local(entry_type: EntryType) -> ExternResult<Vec<(ActionHash, Create, Entry)>> {
   /// Query type
   let query_args = ChainQueryFilter::default()
     .include_entries(true)
     .action_type(ActionType::Create)
     .entry_type(entry_type);
   let records = query(query_args)?;
   /// Get entries for all results
   let mut entries = Vec::new();
   for record in records {
      let RecordEntry::Present(entry) = record.entry() else {
         return zome_error!("Could not convert record");
      };
      let Action::Create(create) = record.action()
        else { panic!("Should be a create Action")};
      entries.push((record.action_address().to_owned(), create.clone(), entry.clone()))
   }
   /// Done
   Ok(entries)
}
