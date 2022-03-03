use hdk::prelude::*;

use zome_delivery_types::*;
use zome_utils::*;

///Find DeliveryNotice with field with given value
#[hdk_extern]
pub fn query_DeliveryNotice(query_field: DeliveryNoticeQueryField) -> ExternResult<Vec<DeliveryNotice>> {
   debug!("*** query_DeliveryNotice() CALLED with {:?}", query_field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ParcelReceived Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(entry_type!(DeliveryNotice)?);
   let notices = query(query_args)?;
   /// Search through query result
   let mut res = Vec::new();
   match query_field {
      DeliveryNoticeQueryField::Sender(sender) => {
         for notice_el in notices {
            let notice: DeliveryNotice = get_typed_from_el(notice_el)?;
            if notice.sender == sender {
               res.push(notice.clone());
            }
         }
      },
      DeliveryNoticeQueryField::Parcel(parcel_eh) => {
         for notice_el in notices {
            let notice: DeliveryNotice = get_typed_from_el(notice_el)?;
            if notice.parcel_summary.parcel_reference.entry_address() == parcel_eh {
               res.push(notice.clone());
            }
         }
      }
   }
   /// Done
   Ok(res)
}


///Find ParcelReceived with field with given value
#[hdk_extern]
pub fn query_ParcelReceived(field: ParcelReceivedQueryField) -> ExternResult<Option<ParcelReceived>> {
   debug!("*** query_ParcelReceived() CALLED with {:?}", field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ParcelReceived Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(entry_type!(ParcelReceived)?);
   let receipts = query(query_args)?;

   match field {
      ParcelReceivedQueryField::Notice(eh) => {
         for receipt_el in receipts {
            let receipt: ParcelReceived = get_typed_from_el(receipt_el)?;
            if receipt.notice_eh == eh {
               return Ok(Some(receipt));
            }
         }
      },
      ParcelReceivedQueryField::Parcel(eh) => {
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