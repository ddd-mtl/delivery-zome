use hdk::prelude::*;
//use zome_utils::*;
use zome_delivery_types::*;


///
pub fn get_delivery_state(_distribution_eh: EntryHash, _recipient: &AgentPubKey) -> ExternResult<DeliveryState> {
   /// FIXME
   Ok(DeliveryState::Unsent)
}


///
pub fn get_destribution_state(_distribution_eh: EntryHash) -> ExternResult<DistributionState> {
   /// FIXME
   Ok(DistributionState::Unsent)
}


///
pub fn get_notice_state(_notice_eh: EntryHash) -> ExternResult<DistributionState> {
   /// FIXME
   Ok(DistributionState::Unsent)
}


// ///
// pub fn get_delivery_state(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<DeliveryState> {
//     /// Look for a DeliveryConfirmation entry
//     let confirmations = get_confirmations(package_eh.clone())?;
//     let confirmed_recipients: Vec<AgentPubKey> = confirmations.iter().map(|x| x.recipient.clone()).collect();
//     if confirmed_recipients.contains(recipient) {
//         return Ok(DeliveryState::Delivered)
//     }
//     /// Look for a Pending/s link
//     let links = get_links(package_eh.clone(), Some(LinkKind::Pending.as_tag()))?;
//     for link in links {
//         /// OutAck
//         if link.tag == LinkKind::Pending.as_tag() {
//             return Ok(DeliveryState::Pending)
//         }
//         /// OutMail
//         let maybe_pendings = LinkKind::Pendings.unconcat_hash(&link.tag);
//         if let Ok(agent) = maybe_pendings {
//             if &agent == recipient {
//                 return Ok(DeliveryState::Pending)
//             }
//         }
//     }
//     /// None found
//     Ok(DeliveryState::Unsent)
// }


//
// /// Get State of InMail
// pub(crate) fn get_inmail_state(inmail_hh: HeaderHash) -> ExternResult<IncomingDeliveryState> {
//     /// Get inMail Details
//     let maybe_details = get_details(inmail_hh.clone(), GetOptions::latest())?;
//     if maybe_details.is_none() {
//         return error("No InMail at given address");
//     }
//     let el_details = match maybe_details.unwrap() {
//         Details::Element(details) => details,
//         Details::Entry(_) => unreachable!("in get_outmail_state()"),
//     };
//     /// Check if deleted
//     if el_details.deletes.len() > 0 {
//         return Ok(IncomingDeliveryState::Deleted);
//     }
//     let inmail: InMail = get_typed_from_el(el_details.element.clone())?;
//     /// Get OutAck
//     let outacks = get_outacks(Some(inmail_hh.to_owned()))?;
//     if outacks.len() < 1 {
//         return Ok(IncomingDeliveryState::ManifestReceived);
//     }
//     /// Determine OutAck delivery state
//     let outack = outacks[0].to_owned();
//     let outack_eh = hash_entry(outack)?;
//     let confirmation_created = try_confirming_pending_ack_has_been_received(outack_eh.clone(), &inmail.from)?;
//     let outack_state = if confirmation_created {
//         DeliveryState::Delivered
//     } else {
//          get_delivery_state(outack_eh, &inmail.from)?
//     };
//     /// Map to inmail state
//     let inmail_state = match outack_state {
//         DeliveryState::Unsent => IncomingDeliveryState::Accepted,
//         DeliveryState::Pending => IncomingDeliveryState::Refused,
//         DeliveryState::Delivered => IncomingDeliveryState::Received,
//     };
//     Ok(inmail_state)
// }