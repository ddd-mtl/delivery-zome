//! Extra callbacks that will be called by the delivery zome

use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::DeliveryEntry;
use zome_delivery_types::*;


pub const COMMIT_PARCEL_CALLBACK_NAME: &'static str = "commit_parcel";



/// Call The Zome owner of the entry to commit it, since only that zome is allowed to commit one of its entries?
///
pub fn call_commit_parcel(entry: Entry, notice: &DeliveryNotice, maybe_link_ah: Option<ActionHash>)
   -> ExternResult<ActionHash>
{

   let input = CommitParcelInput {
      zome_index: notice.summary.parcel_reference.zome_index(),
      entry_index: notice.summary.parcel_reference.entry_index(),
      entry_visibility: notice.summary.parcel_reference.entry_visibility(),
      entry: entry.clone(),
      maybe_link_ah: maybe_link_ah.clone(),
   };

   /// Make sure CreateLink exists
   if let Some(link_hh) = maybe_link_ah {
      let maybe_el = get(link_hh.clone(), GetOptions::default())?;
      if maybe_el.is_none() {
         return zome_error!("call_commit_parcel(): CreateLink not found.");
      }
   }

   debug!("call_commit_parcel() zome_names = {:?}", dna_info()?.zome_names);
   let zome_name = dna_info()?.zome_names[notice.summary.parcel_reference.zome_index().0 as usize].clone();
   debug!("call_commit_parcel()  zome_name = {}", zome_name);
   let response = call_remote(
      agent_info()?.agent_latest_pubkey,
      DELIVERY_ZOME_NAME,//zome_name,
      COMMIT_PARCEL_CALLBACK_NAME.into(),
      None,
      input.clone(),
   )?;
   let ah = decode_response(response)?;

   // /// Delete Link
   // if let Some(link_hh) = input.maybe_link_hh {
   //    debug!("call_commit_parcel() delete_link {:?}", link_hh);
   //        let input = DeleteLinkInput::new(link_hh,
   //           ChainTopOrdering::Relaxed,
   //        );
   //     let _hh = HDK.with(|h| {
   //         h.borrow()
   //          .delete_link(input)
   //     })?;
   // }

   /// Create ReceptionProof if its an AppEntry
   /// (for a Manifest, we have to wait for all chunks to be received)
   if let ParcelReference::AppEntry(..) = notice.summary.parcel_reference {
      let received = ReceptionProof {
         notice_eh: hash_entry(notice.clone())?,
         parcel_eh: hash_entry(entry.clone())?,
      };
      let response = call_self("commit_ReceptionProof", received.clone())?;
      let received_eh: EntryHash = decode_response(response)?;
      debug!("call_commit_parcel() received_eh = {:?}", received_eh);
   }
   /// Done
   Ok(ah)
}


///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitParcelInput {
   pub zome_index: ZomeIndex,
   pub entry_index: EntryDefIndex,
   pub entry_visibility: EntryVisibility,
   pub entry: Entry,
   pub maybe_link_ah: Option<ActionHash>,
}


/// Zome Function Callback required by delivery-zome.
/// Commits an arbitrary entry to source-chain.
/// Should not be called directly. Only via remote call to self.
/// Name of this function must equal COMMIT_PARCEL_CALLBACK_NAME global constant.
#[hdk_extern]
fn commit_parcel(input: CommitParcelInput) -> ExternResult<ActionHash> {
   debug!("commit_parcel() entry_def_id = {:?}:{:?} | {}", input.entry_index, input.zome_index, zome_info()?.name);
   /// Create CreateInput
   let create_input = CreateInput::new(
      EntryDefLocation::App(AppEntryDefLocation {
         zome_index: input.zome_index,
         entry_def_index: input.entry_index,
      }),
      input.entry_visibility,
      input.entry,
      ChainTopOrdering::Relaxed, // Strict //Relaxed
   );
   /// Commit Parcel
   let parcel_ah = create(create_input)?;
   /// Delete Link
   if let Some(link_ah) = input.maybe_link_ah {
      debug!("commit_parcel() delete_link: {:?}", link_ah);
      /// Make sure CreateLink exists
      let maybe_el = get(link_ah.clone(), GetOptions::default())?;
      if maybe_el.is_none() {
         return zome_error!("CreateLink not found.");
      }
      /// Delete
      let input = DeleteLinkInput::new(link_ah,
                                       ChainTopOrdering::Relaxed,
      );
      let _hh = HDK.with(|h| {
         h.borrow()
          .delete_link(input)
      })?;
       // let _hh = delete_link(link_hh)?;
   }
   /// Done
   Ok(parcel_ah)
}


///
#[hdk_extern]
fn commit_NoticeAck(ack: NoticeAck) -> ExternResult<ActionHash> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   return create_entry_relaxed(DeliveryEntry::NoticeAck(ack));
}



///
#[hdk_extern]
fn commit_ReceptionProof(pr: ReceptionProof) -> ExternResult<EntryHash> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let eh = hash_entry(pr.clone())?;
   let _hh = create_entry_relaxed(DeliveryEntry::ReceptionProof(pr))?;
   return Ok(eh);
}
