//! Extra callbacks that will be called by the delivery zome

use hdk::prelude::*;

use zome_utils::*;
use zome_delivery_integrity::DeliveryEntry;
use zome_delivery_types::NoticeReceived;


pub const COMMIT_PARCEL_CALLBACK_NAME: &'static str = "commit_parcel";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitParcelInput {
   pub zome_index: u8, //ZomeIndex,
   pub entry_index: u8, // EntryDefIndex,
   pub entry_visibility: EntryVisibility,
   pub entry: Entry,
   pub maybe_link_ah: Option<ActionHash>,
}

/// Zome Function Callback required by delivery-zome.
/// Should not be called directly. Only via remote call to self.
/// Name of this function must equal COMMIT_PARCEL_CALLBACK_NAME global constant.
#[hdk_extern]
fn commit_parcel(input: CommitParcelInput) -> ExternResult<ActionHash> {
   debug!("commit_parcel() entry_def_id = {:?} | {}", input.entry_index, zome_info()?.name);
   /// Create CreateInput
   let create_input = CreateInput::new(
      EntryDefLocation::App(AppEntryDefLocation {
         zome_index: ZomeIndex::from(input.zome_index),
         entry_def_index: EntryDefIndex::from(input.entry_index),
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



#[hdk_extern]
fn commit_NoticeReceived(ack: NoticeReceived) -> ExternResult<ActionHash> {
   return create_entry_relaxed(DeliveryEntry::NoticeReceived(ack));
}