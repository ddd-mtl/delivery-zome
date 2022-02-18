use hdk::prelude::*;

use crate::{
   self::*,
   entries::*,
   dm_protocol::*,
   dm::*,
};

///
pub(crate) fn send_item(
   recipient: AgentPubKey,
   distribution_eh: EntryHash,
   pending_item: PendingItem,
   signed_item: Signature,
) -> ExternResult<SendSuccessKind> {
   debug!("send_item() START - {:?}", recipient);

   // /// Shortcut to self
   // let me = agent_info()?.agent_latest_pubkey;
   // if recipient.clone() == me {
   //    debug!("request_reception() Self");
   //    let msg = MailMessage {
   //       outmail_eh: eh.clone(),
   //       mail: parcel_description.clone(),
   //       mail_signature: sender_description_signature.clone(),
   //    };
   //    let inmail = InMail::from_direct(msg, me.clone());
   //    debug!("request_reception() REMOTE CALLING...");
   //    let res = call_remote(
   //       me,
   //       zome_info()?.name,
   //       "commit_inmail".to_string().into(),
   //       None,
   //       inmail,
   //    )?;
   //    debug!("commit_inmail() END : {:?}", res);
   //    assert!(matches!(res, ZomeCallResponse::Ok { .. }));
   //    return Ok(SendSuccessKind::OK_SELF);
   // }

   //     if let DeliveryProtocol::Success(_) = response_dm {
//         return Ok(());
//     }


   /// Try sending directly to other Agent if Online
   // let result = send_item_by_dm(recipient, distribution_eh, pending_item.clone(), signed_item);
   let response_dm = send_dm(recipient, DeliveryProtocol::Item(pending_item.clone()))?;
   debug!("send_item_by_dm() response_dm = {:?}", response_dm);
   if let DeliveryProtocol::Success(_) = response_dm {
      return Ok(SendSuccessKind::OK_DIRECT);
   } else {
      let err = result.err().unwrap();
      debug!("send_item() failed: {:?}", err);
   }

   debug!("send_item() - Commit PendingItem...");
   /// DM failed, send to DHT instead by creating a PendingMail
   /// Create and commit PendingMail with remote call to self

   let input = CommitPendingItemInput {
      item: pending_item,
      recipient: recipient.clone(),
   };
   debug!("send_item() - calling commit_pending_mail()");
   let response = call_remote(
      me,
      zome_info()?.name,
      "commit_pending_item".to_string().into(),
      None,
      input,
   )?;
   debug!("send_confirmation() - commit_pending_mail() response: {:?}", response);
   return match response {
      ZomeCallResponse::Ok(_) => Ok(SendSuccessKind::OK_PENDING),
      _ => error("call_remote() to commit_pending_item() failed")
   };
}
