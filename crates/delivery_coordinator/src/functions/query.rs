use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;


///
#[hdk_extern]
pub fn query_all_Distribution(_: ()) -> ExternResult<Vec<(ActionHash, Timestamp, Distribution)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<Distribution>(DeliveryEntryTypes::Distribution.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(ah, create, typed)| (ah, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


// ///Find Distribution with field with given value
// #[hdk_extern]
// pub fn query_Distribution(_: ()) -> ExternResult<Vec<(EntryHash, Distribution)>> {
//    std::panic::set_hook(Box::new(zome_panic_hook));
//    /// Get all Create Distribution Elements with query
//    let tuples = get_all_typed_local::<Distribution>(DeliveryEntryTypes::Distribution.try_into().unwrap())?;
//    let res = tuples.into_iter().map(|(_ah, _create, distrib)| {
//       let eh = hash_entry(distrib.clone()).unwrap();
//       (eh, distrib)
//    }).collect();
//    Ok(res)
// }


///
#[hdk_extern]
pub fn query_all_DeliveryNotice(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, DeliveryNotice)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<DeliveryNotice>(DeliveryEntryTypes::DeliveryNotice.try_into().unwrap())?;
   let res = tuples.into_iter()
      .map(|(_, create, notice)| (create.entry_hash, create.timestamp, notice))
      .collect();
   /// Done
   Ok(res)
}


///Find DeliveryNotice with field with given value
#[hdk_extern]
pub fn query_DeliveryNotice(query_field: DeliveryNoticeQueryField) -> ExternResult<Vec<(DeliveryNotice, Timestamp)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("query_DeliveryNotice() CALLED with {:?}", query_field);

   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<DeliveryNotice>(DeliveryEntryTypes::DeliveryNotice.try_into().unwrap())?;
   let len = tuples.len();
   /// Search through query result
   let mut res = Vec::new();
   match query_field {
      DeliveryNoticeQueryField::Sender(sender) => {
         for (_ah, create, notice) in tuples {
            if notice.sender == sender {
               res.push((notice.clone(), create.timestamp));
            }
         }
      },
      DeliveryNoticeQueryField::Parcel(parcel_eh) => {
         for (_ah, create, notice) in tuples {
            if notice.summary.parcel_reference.eh == parcel_eh {
               res.push((notice.clone(), create.timestamp));
            }
         }
      },
      DeliveryNoticeQueryField::Distribution(distrib_ah) => {
         for (_ah, create, notice) in tuples {
            if notice.distribution_ah == distrib_ah {
               res.push((notice.clone(), create.timestamp));
            }
         }
         if res.len() > 1 {
            error!("More than one Notice found for distribution");
         }
      }
   }
   debug!("query_DeliveryNotice() found {} notice(s) ", len);
   //debug!("query_DeliveryNotice() res {:?}", res);
   /// Done
   Ok(res)
}


///
#[hdk_extern]
pub fn query_all_NoticeAck(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, NoticeAck)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<NoticeAck>(DeliveryEntryTypes::NoticeAck.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


/// Find NoticeAck with field with given value
#[hdk_extern]
pub fn query_NoticeAck(field: NoticeAckQueryField) -> ExternResult<Vec<NoticeAck>> {
   //debug!("query_NoticeAck() CALLED with {:?}", field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create NoticeAck Elements with query
   let tuples = get_all_typed_local::<NoticeAck>(DeliveryEntryTypes::NoticeAck.try_into().unwrap())?;
   //debug!(" - tuples len: {:?}", tuples.len());
   debug!("*** query_NoticeAck() tuples count: {}", tuples.len());
   /// Search through query result
   let mut res = Vec::new();
   match field {
      NoticeAckQueryField::Recipient(agent) => {
         for (_, _, received) in tuples {
            if received.recipient == agent {
               res.push(received);
            }
         }
      },
      NoticeAckQueryField::Distribution(ah) => {
         for (_, _, received) in tuples {
            if received.distribution_ah == ah {
               res.push(received);
            }
         }
      }
   }
   debug!("*** query_NoticeAck() res count: {}", res.len());
   /// Done
   Ok(res)
}


///
#[hdk_extern]
pub fn query_all_NoticeReply(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, NoticeReply)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<NoticeReply>(DeliveryEntryTypes::NoticeReply.try_into().unwrap())?;
   let res = tuples.into_iter()
       .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
       .collect();
   /// Done
   Ok(res)
}


///Find NoticeReply with given notice_eh value
pub fn query_NoticeReply(notice_eh: EntryHash) -> ExternResult<Option<NoticeReply>> {
   //debug!("*** query_NoticeReply() CALLED with {:?}", notice_eh);
   /// Get all Create NoticeReply Elements with query
   let tuples = get_all_typed_local::<NoticeReply>(DeliveryEntryTypes::NoticeReply.try_into().unwrap())?;
   /// Search through query result
   for (_, _, reply) in tuples {
      if reply.notice_eh == notice_eh {
         return Ok(Some(reply));
      }
   }
   /// Done
   Ok(None)
}


///
#[hdk_extern]
pub fn query_all_ReplyAck(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ReplyAck)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<ReplyAck>(DeliveryEntryTypes::ReplyAck.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


///Find ReplyAck with field with given value
pub fn query_ReplyAck(maybe_distribution: Option<ActionHash>, maybe_recipient: Option<AgentPubKey>)
   -> ExternResult<Vec<ReplyAck>> {
   //std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("*** query_ReplyAck() CALLED");
   /// Get all Create Elements with query
   let tuples = get_all_typed_local::<ReplyAck>(DeliveryEntryTypes::ReplyAck.try_into().unwrap())?;
   let mut typeds: Vec<ReplyAck> = tuples.into_iter().map(|(_,_,x)| x).collect();
   /// Search through query result
   if let Some(distrib_ah) = maybe_distribution {
      typeds.retain(|r| r.distribution_ah == distrib_ah);
   }
   if let Some(recipient) = maybe_recipient {
      typeds.retain(|r| r.recipient == recipient);
   }
   /// Done
   Ok(typeds)
}


///
#[hdk_extern]
pub fn query_all_ReceptionProof(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ReceptionProof)>> {
   //debug!("*** query_ReceptionProof() CALLED with {:?}", query_field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let tuples = get_all_typed_local::<ReceptionProof>(DeliveryEntryTypes::ReceptionProof.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


///Find ReceptionProof with field with given value
#[hdk_extern]
pub fn query_ReceptionProof(field: ReceptionProofQueryField) -> ExternResult<Option<(EntryHash, Timestamp, ReceptionProof)>> {
   debug!("*** query_ReceptionProof() CALLED with {:?}", field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ReceptionProof Elements with query
   let tuples = get_all_typed_local::<ReceptionProof>(DeliveryEntryTypes::ReceptionProof.try_into().unwrap())?;
   //debug!("*** query_ReceptionProof() tuples: {:?}", tuples.clone());
   /// Search through query result
   match field {
      ReceptionProofQueryField::Notice(eh) => {
         for (_ah, create, reception) in tuples {
            if reception.notice_eh == eh {
               return Ok(Some((create.entry_hash, create.timestamp, reception)));
            }
         }
      },
      ReceptionProofQueryField::Parcel(eh) => {
         for (_ah, create, reception) in tuples {
            //debug!("*** query_ReceptionProof() Parcel  receipt.parcel_eh {:?}", receipt.parcel_eh);
            if reception.parcel_eh == eh {
               return Ok(Some((create.entry_hash, create.timestamp, reception)));
            }
         }
      },
   }
   /// Done
   Ok(None)
}


///
#[hdk_extern]
pub fn query_all_ReceptionAck(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ReceptionAck)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create ReceptionAck Elements with query
   let tuples = get_all_typed_local::<ReceptionAck>(DeliveryEntryTypes::ReceptionAck.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


/// Find ReceptionAck with field with given value
pub fn query_ReceptionAck(maybe_distribution: Option<ActionHash>, maybe_recipient: Option<AgentPubKey>)
   -> ExternResult<Vec<ReceptionAck>> {
   //std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("*** query_ReceptionAck() CALLED");
   /// Get all Create ReceptionAck Elements with query
   let tuples = get_all_typed_local::<ReceptionAck>(DeliveryEntryTypes::ReceptionAck.try_into().unwrap())?;
   let mut receipts: Vec<ReceptionAck> = tuples.into_iter().map(|(_,_,x)| x).collect();
   //debug!("*** query_DeliveryReceipt() receipts count: {}", receipts.len());
   /// Search through query result
   if let Some(distrib_ah) = maybe_distribution {
      receipts.retain(|r| r.distribution_ah == distrib_ah);
   }
   //debug!("*** query_DeliveryReceipt() receipts distrib: {}", receipts.len());
   if let Some(recipient) = maybe_recipient {
      receipts.retain(|r| r.recipient == recipient);
   }
   //debug!("*** query_DeliveryReceipt() receipts recipient: {}", receipts.len());
   /// Done
   Ok(receipts)
}


///
#[hdk_extern]
pub fn query_all_private_manifests(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ParcelManifest)>> {
   //debug!("*** query_all_Manifest() CALLED with {:?}", query_field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create Elements with query
   let tuples = get_all_typed_local::<ParcelManifest>(DeliveryEntryTypes::PrivateManifest.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


///
#[hdk_extern]
pub fn query_all_public_manifests(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ParcelManifest)>> {
   //debug!("*** query_all_Manifest() CALLED with {:?}", query_field);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create Elements with query
   let tuples = get_all_typed_local::<ParcelManifest>(DeliveryEntryTypes::PublicManifest.try_into().unwrap())?;
   let res = tuples.into_iter()
                   .map(|(_, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}



///
#[hdk_extern]
pub fn query_all_public_chunks(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ParcelChunk)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let entry_type = DeliveryEntryTypes::PublicChunk.try_into().unwrap();
   debug!("PublicChunk entry_type: {:?}", entry_type);
   let tuples = get_all_typed_local::<ParcelChunk>(entry_type)?;
   let res = tuples.into_iter()
                   .map(|(_ah, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}


///
#[hdk_extern]
pub fn query_all_private_chunks(_: ()) -> ExternResult<Vec<(EntryHash, Timestamp, ParcelChunk)>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get all Create DeliveryNotice Elements with query
   let entry_type = DeliveryEntryTypes::PrivateChunk.try_into().unwrap();
   debug!("PrivateChunk entry_type: {:?}", entry_type);
   let tuples = get_all_typed_local::<ParcelChunk>(entry_type)?;
   let res = tuples.into_iter()
                   .map(|(_ah, create, typed)| (create.entry_hash, create.timestamp, typed))
                   .collect();
   /// Done
   Ok(res)
}
