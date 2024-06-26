use hdk::prelude::*;
use zome_utils::*;
use zome_signals::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;


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
   query_all_typed::<Distribution>(DeliveryEntryTypes::Distribution.try_into().unwrap())?;
   Ok(())
}


///
#[hdk_extern]
pub fn query_all_DeliveryNotice(_: ()) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   query_all_typed::<DeliveryNotice>(DeliveryEntryTypes::DeliveryNotice.try_into().unwrap())?;
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
