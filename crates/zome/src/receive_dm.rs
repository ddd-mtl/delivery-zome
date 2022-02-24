use hdk::prelude::*;
use zome_delivery_types::utils::*;

use crate::DeliveryProtocol;
use zome_delivery_types::*;
use crate::functions::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectMessage {
    pub from: AgentPubKey,
    pub msg: DeliveryProtocol,
}

/// Start point for any remote call in this zome
/// WARN: Name of function must match REMOTE_ENDPOINT const value
#[hdk_extern]
pub fn receive_delivery_dm(dm: DirectMessage) -> ExternResult<DeliveryProtocol> {
    debug!("Received from: {}", dm.from);
    let reply = match dm.msg {
        DeliveryProtocol::ChunkRequest(_chunk_eh) => { /* FIXME */ DeliveryProtocol::Pong },
        DeliveryProtocol::ParcelRequest(distribution_eh) => {
            receive_dm_parcel_request(dm.from, distribution_eh)
        },
        DeliveryProtocol::Item(item) => {
            match item.kind {
                ItemKind::DeliveryNotice => receive_dm_notice(dm.from, item),
                ItemKind::DeliveryReply  => receive_dm_reply(dm.from, item),
                //PendingKind::Entry => {/* FIXME */},
                //PendingKind::ReceptionConfirmation => receive_dm_reception(from, item),
                _ => panic!("FIXME kind not supported yet"),
            }
        },
        DeliveryProtocol::Ping => DeliveryProtocol::Pong,
        _ => {
             DeliveryProtocol::Failure("Unexpected protocol".to_owned())
        },
    };
    Ok(reply)
}


/// Returns Success or Failure
pub fn receive_dm_parcel_request(from: AgentPubKey, distribution_eh: EntryHash) -> DeliveryProtocol {
    /// Get Distribution Entry
    let maybe_distribution: ExternResult<Distribution> = get_typed_from_eh(distribution_eh);
    if let Err(err) = maybe_distribution {
        let response_str = "Distribution not found";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let distribution = maybe_distribution.unwrap();
    /// Make sure DM is from a valid recipient
    if !distribution.recipients.contains(&from) {
        let response_str = "Request from invalid recipient";
        debug!("{}", response_str);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Get Parcel entry hash
    let parcel_eh = distribution.parcel_summary.reference.entry_address();
    /// Get entry
    let maybe_maybe_element = get(parcel_eh, GetOptions::content());
    if let Err(err) = maybe_maybe_element {
        let response_str = "Parcel Entry not found";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let element = match maybe_maybe_element.unwrap() {
        None => {
            let response_str = "Parcel Entry not found";
            debug!("{}", response_str);
            return DeliveryProtocol::Failure(response_str.to_string());
        },
        Some(el) => el,
    };
    /// Return Entry
    let maybe_maybe_entry= element.entry().to_app_option::<Entry>();
    if let Err(err) = maybe_maybe_entry {
        let response_str = "Entry not found in Element";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    return DeliveryProtocol::ParcelResponse(maybe_maybe_entry.unwrap().unwrap());
}


/// Commit received DeliveryNotice from sender
/// Returns Success or Failure
pub fn receive_dm_notice(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_maybe_notice: ExternResult<Option<DeliveryNotice>> = unpack_item(item, from.clone());
    if let Err(err) = maybe_maybe_notice {
        let response_str = "Failed deserializing DeliveryNotice";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let maybe_notice = maybe_maybe_notice.unwrap();
    if maybe_notice.is_none() {
        let response_str = "Failed deserializing DeliveryNotice 2";
        debug!("{}", response_str);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Commit DeliveryNotice
    let maybe_hh = create_entry(&maybe_notice.unwrap());
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryNotice";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success("DeliveryNotice received".to_string());
}

/// Create and commit a ReplyReceived from a DeliveryReply
/// Returns Success or Failure
pub fn receive_dm_reply(from: AgentPubKey, pending_item: PendingItem) -> DeliveryProtocol {
    let maybe_maybe_reply: ExternResult<Option<DeliveryReply>> = unpack_item(pending_item.clone(), from.clone());
    if let Err(err) = maybe_maybe_reply {
        let response_str = "Failed deserializing DeliveryReply";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let maybe_reply = maybe_maybe_reply.unwrap();
    if maybe_reply.is_none() {
        let response_str = "Failed deserializing DeliveryReply 2";
        debug!("{}", response_str);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Create ReplyReceived
    let receipt = ReplyReceived {
        distribution_eh: pending_item.distribution_eh,
        recipient: from,
        recipient_signature: pending_item.author_signature,
        has_accepted: maybe_reply.unwrap().has_accepted,
        //date: now(),
    };
    /// Commit ReplyReceived
    let maybe_hh = create_entry(&receipt);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing ReplyReceived";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success("DeliveryReply received".to_string());
}


// /// Returns Success or Failure
// pub fn receive_dm_reception(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
//     let maybe_recepetion: ExternResult<Option<ReceptionConfirmation>> = item.try_into(from.clone());
//     if let Err(err) = maybe_recepetion {
//         let response_str = "Failed deserializing ReceptionConfirmation";
//         debug!("{}: {}", response_str, err);
//         return DeliveryProtocol::Failure(response_str.to_string());
//     }
//     /// Create DeliveryConfirmation
//     let confirmation = DeliveryConfirmation {
//         accepted_parcel: maybe_recepetion.unwrap().unwrap().reception_response,
//         distribution_eh: item.distribution_eh,
//         recipient: from,
//         recipient_parcel_signature: item.author_signature,
//         date_of_response: now(),
//     };
//     /// Commit DeliveryConfirmation
//     let maybe_hh = create_entry(&confirmation);
//     if let Err(err) = maybe_hh {
//         let response_str = "Failed committing DeliveryConfirmation";
//         debug!("{}: {}", response_str, err);
//         return DeliveryProtocol::Failure(response_str.to_string());
//     }
//     /// Return Success
//     return DeliveryProtocol::Success("DeliveryConfirmation received".to_string());
// }