
use hdk::prelude::*;
use crate::{send_item::*, entries::*, utils::*, EntryKind};


/// Entry for confirming a delivery has been well received or refused by a recipient
/// TODO: This should be a private link instead of an entry
#[hdk_entry(id = "ParcelReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelReceived {
   pub notice_eh: EntryHash,
   pub parcel_eh: EntryHash,
   //pub signed_parcel: SignedHeaderHashed, // signed header of parcel's Element
}

pub enum ParcelReceivedField {
   Notice(EntryHash),
   Parcel(EntryHash)
}

impl ParcelReceived {
   ///Find ParcelReceived with field with given value
   pub fn query(field: ParcelReceivedField) -> ExternResult<Option<ParcelReceived>> {
      /// Get all Create ParcelReceived Elements with query
      let query_args = ChainQueryFilter::default()
         .include_entries(true)
         .header_type(HeaderType::Create)
         .entry_type(EntryKind::ParcelReceived.as_type());
      let receipts = query(query_args)?;

      match field {
         ParcelReceivedField::Notice(eh) => {
            for receipt_el in receipts {
               let receipt: ParcelReceived = get_typed_from_el(receipt_el)?;
               if receipt.notice_eh == eh {
                  return Ok(Some(receipt));
               }
            }
         },
         ParcelReceivedField::Parcel(eh) => {
            for receipt_el in receipts {
               let receipt: ParcelReceived = get_typed_from_el(receipt_el)?;
               if receipt.parcel_eh == eh {
                  return Ok(Some(receipt));
               }
            }
         },
      }
      /// Done
      Ok(None)
   }

   ///
   pub fn post_commit(receipt_eh: &EntryHash, reception: Self) -> ExternResult<()>
   {
      debug!("post_commit_ParcelReceived() {:?}", receipt_eh);
      /// Get DeliveryNotice
      let notice: DeliveryNotice = get_typed_from_eh(reception.notice_eh.clone())?;
      /// Sign Item
      let signature = sign(agent_info()?.agent_latest_pubkey, reception.clone())?;
      /// Create PendingItem
      let pending_item = PendingItem::from_reception(
         reception.clone(),
         notice.distribution_eh.clone(),
         notice.sender.clone(),
      )?;
      /// Send it to recipient
      let _ = send_item(
         notice.sender,
         notice.distribution_eh.clone(),
         pending_item,
         signature)?;
      /// Done
      Ok(())
   }
}


