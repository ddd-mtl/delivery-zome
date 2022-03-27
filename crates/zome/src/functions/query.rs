use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;


///Find DeliveryNotice with field with given value
#[hdk_extern]
pub fn query_DeliveryNotice(query_field: DeliveryNoticeQueryField) -> ExternResult<Vec<DeliveryNotice>> {
   //debug!("*** query_DeliveryNotice() CALLED with {:?}", query_field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let notices: Vec<DeliveryNotice> = get_all_typed_local(entry_type!(DeliveryNotice)?)?;
   /// Search through query result
   let mut res = Vec::new();
   match query_field {
      DeliveryNoticeQueryField::Sender(sender) => {
         for notice in notices {
            if notice.sender == sender {
               res.push(notice.clone());
            }
         }
      },
      DeliveryNoticeQueryField::Parcel(parcel_eh) => {
         for notice in notices {
            if notice.summary.parcel_reference.entry_address() == parcel_eh {
               res.push(notice.clone());
            }
         }
      },
      DeliveryNoticeQueryField::Distribution(distrib_eh) => {
         if notices.len() > 1 {
            error!("More than one Notice found for distribution")
         }
         if notices[0].distribution_eh == distrib_eh {
            res.push(notices[0].clone());
         }
      }
   }
   /// Done
   Ok(res)
}



///Find NoticeReceived with field with given value
#[hdk_extern]
pub fn query_NoticeReceived(field: NoticeReceivedQueryField) -> ExternResult<Vec<NoticeReceived>> {
   //debug!("*** query_NoticeReceived() CALLED with {:?}", field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create NoticeReceived Elements with query
   let receiveds: Vec<NoticeReceived> = get_all_typed_local(entry_type!(NoticeReceived)?)?;
   /// Search through query result
   let mut res = Vec::new();
   match field {
      NoticeReceivedQueryField::Recipient(agent) => {
         for received in receiveds {
            if received.recipient == agent {
               res.push(received);
            }
         }
      },
      NoticeReceivedQueryField::Distribution(eh) => {
         for received in receiveds {
            if received.distribution_eh == eh {
               res.push(received);
            }
         }
      },
   }
   /// Done
   Ok(res)
}



///Find DeliveryReply with given notice_eh value
pub fn query_DeliveryReply(notice_eh: EntryHash) -> ExternResult<Option<DeliveryReply>> {
   //debug!("*** query_DeliveryReply() CALLED with {:?}", notice_eh);
   /// Get all Create DeliveryReply Elements with query
   let replies: Vec<DeliveryReply> = get_all_typed_local(entry_type!(DeliveryReply)?)?;
   if replies.len() > 1 {
      error!("More than one reply found for DeliveryNotice {:?}", notice_eh);
   }
   /// Search through query result
   for reply in replies {
      return Ok(Some(reply));
   }
   /// Done
   Ok(None)
}


///Find ReplyReceived with field with given value
pub fn query_ReplyReceived(maybe_distribution: Option<EntryHash>, maybe_recipient: Option<AgentPubKey>)
   -> ExternResult<Vec<ReplyReceived>> {
   //std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("*** query_ReplyReceived() CALLED");
   /// Get all Create DeliveryReceipt Elements with query
   let mut typeds: Vec<ReplyReceived> = get_all_typed_local(entry_type!(ReplyReceived)?)?;
   /// Search through query result
   if let Some(distrib_eh) = maybe_distribution {
      typeds.retain(|r| r.distribution_eh == distrib_eh);
   }
   if let Some(recipient) = maybe_recipient {
      typeds.retain(|r| r.recipient == recipient);
   }
   /// Done
   Ok(typeds)
}


///Find ParcelReceived with field with given value
#[hdk_extern]
pub fn query_ParcelReceived(field: ParcelReceivedQueryField) -> ExternResult<Option<ParcelReceived>> {
   //debug!("*** query_ParcelReceived() CALLED with {:?}", field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ParcelReceived Elements with query
   let receipts: Vec<ParcelReceived> = get_all_typed_local(entry_type!(ParcelReceived)?)?;
   //debug!("*** query_ParcelReceived() CALLED with {:?}", field);
   /// Search through query result
   match field {
      ParcelReceivedQueryField::Notice(eh) => {
         for receipt in receipts {
            if receipt.notice_eh == eh {
               return Ok(Some(receipt));
            }
         }
      },
      ParcelReceivedQueryField::Parcel(eh) => {
         for receipt in receipts {
            //debug!("*** query_ParcelReceived() Parcel  receipt.parcel_eh {:?}", receipt.parcel_eh );
            if receipt.parcel_eh == eh {
               return Ok(Some(receipt));
            }
         }
      },
   }
   /// Done
   Ok(None)
}


///Find NoticeReceived with field with given value
pub fn query_DeliveryReceipt(maybe_distribution: Option<EntryHash>, maybe_recipient: Option<AgentPubKey>)
   -> ExternResult<Vec<DeliveryReceipt>> {
   //std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("*** query_DeliveryReceipt() CALLED");
   /// Get all Create DeliveryReceipt Elements with query
   let mut receipts: Vec<DeliveryReceipt> = get_all_typed_local(entry_type!(DeliveryReceipt)?)?;
   /// Search through query result
   if let Some(distrib_eh) = maybe_distribution {
      receipts.retain(|r| r.distribution_eh == distrib_eh);
   }
   if let Some(recipient) = maybe_recipient {
      receipts.retain(|r| r.recipient == recipient);
   }
   /// Done
   Ok(receipts)
}
