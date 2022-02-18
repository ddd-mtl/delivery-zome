use hdk::prelude::*;

use crate::{
   self::*,
   entries::*,
   dm_protocol::*,
   dm::*,
};


///
pub(crate) fn send_parcel_description(
   recipient: AgentPubKey,
   distribution_eh: EntryHash,
   parcel_description: ParcelDescription,
   sender_description_signature: Signature,
) -> ExternResult<SendSuccessKind> {
   debug!("request_reception() START - {:?}", recipient);


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


   /// Try sending directly to other Agent if Online
   let result = send_parcel_description_by_dm(recipient, distribution_eh, parcel_description, sender_description_signature);
   if result.is_ok() {
      return Ok(SendSuccessKind::OK_DIRECT);
   } else {
      let err = result.err().unwrap();
      debug!("request_reception() failed: {:?}", err);
   }


   debug!("request_reception() - Creating PendingItem...");
   /// DM failed, send to DHT instead by creating a PendingMail
   /// Create and commit PendingMail with remote call to self
   let pending_item = PendingItem::from_description(
      parcel_description.clone(),
      distribution_eh.clone(),
      recipient.clone(),
   )?;
   let input = CommitPendingItemInput {
      item: pending_item,
      recipient: recipient.clone(),
   };
   debug!("request_reception() - calling commit_pending_mail()");
   let response = call_remote(
      me,
      zome_info()?.name,
      "commit_pending_item".to_string().into(),
      None,
      input,
   )?;
   debug!("request_reception() - commit_pending_mail() response: {:?}", response);
   return match response {
      ZomeCallResponse::Ok(_) => Ok(SendSuccessKind::OK_PENDING),
      _ => error("call_remote() to commit_pending_item() failed")
   };
}



/// Attempt sending reception request via DM
fn send_parcel_description_by_dm(
   recipient: AgentPubKey,
   sender_distribution_eh: EntryHash,
   description: ParcelDescription,
   sender_description_signature: Signature,
) -> ExternResult<()> {
   /// --  Send Mail
   debug!("request_reception_by_dm() to {}", recipient);
   /// Create DM
   let msg = ReceptionRequestMessage {
      description,
      sender_description_signature,
      sender_distribution_eh,
   };
   /// Send DM
   let response_dm = send_dm(recipient, DeliveryProtocol::ReceptionRequest(msg))?;
   debug!("request_reception_by_dm() response_dm = {:?}", response_dm);
   /// Check Response
   if let DeliveryProtocol::Success(_) = response_dm {
      return Ok(());
   }
   return error(&format!("request_reception_by_dm() failed: {:?}", response_dm));
}