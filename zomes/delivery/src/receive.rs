use hdk::prelude::*;

use crate::{
    DeliveryProtocol,
    //signal_protocol::*,
    utils::*,
    entries::*,
    parcel::*,
};

/// Start point for any remote call in this zome
/// WARN: Name of function must match REMOTE_ENDPOINT const value
#[hdk_extern]
pub fn receive_delivery_dm(from: AgentPubKey, dm: DeliveryProtocol) -> DeliveryProtocol {
    debug!("Received from: {}", from);
    match dm {
        DeliveryProtocol::ParcelRequest(distribution_eh) => receive_dm_parcel_request(from, distribution_eh),
        DeliveryProtocol::Item(item) => {
            match item.app_type {
                PendingKind::DeliveryNotice => receive_dm_notice(from, item),
                PendingKind::DeliveryReply  => receive_dm_reply(from, item),
                PendingKind::Entry => {/* FIXME */},
                //PendingKind::ReceptionConfirmation => receive_dm_reception(from, item),
            }
        },
        DeliveryProtocol::Ping => DeliveryProtocol::Pong,
        _ => {
             DeliveryProtocol::Failure("Unexpected protocol".to_owned())
        },
    }
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
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Get Parcel entry hash
    let parcel = distribution.parcel_description.parcel;
    let parcel_eh = parcel.entry_address();
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
            debug!("{}: {}", response_str, err);
            return DeliveryProtocol::Failure(response_str.to_string());
        },
        Some(el) => el,
    };
    /// Return Entry
    return DeliveryProtocol::ParcelResponse(element.entry().unwrap());
}


/// Returns Success or Failure
pub fn receive_dm_notice(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_maybe_notice: ExternResult<Option<DeliveryNotice>> = item.try_into(from.clone());
    if let Err(err) = maybe_maybe_notice {
        let response_str = "Failed deserializing DeliveryNotice";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let maybe_notice = maybe_maybe_notice.unwrap();
    if maybe_notice.is_none() {
        let response_str = "Failed deserializing DeliveryNotice 2";
        debug!("{}: {}", response_str, err);
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


/// Returns Success or Failure
pub fn receive_dm_reply(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_notification: ExternResult<Option<DeliveryNotification>> = item.try_into(from.clone());
    if let Err(err) = maybe_notification {
        let response_str = "Failed deserializing DeliveryNotification";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Create DeliveryConfirmation
    let confirmation = DescriptionConfirmation {
        distribution_eh: item.distribution_eh,
        recipient: from,
        recipient_manifest_signature: item.author_signature,
        date_of_reception: now(),
    };
    /// Commit DeliveryConfirmation
    let maybe_hh = create_entry(&confirmation);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryConfirmation";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success("DeliveryNotification received".to_string());
}


/// Returns Success or Failure
pub fn receive_dm_reception(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_recepetion: ExternResult<Option<ReceptionConfirmation>> = item.try_into(from.clone());
    if let Err(err) = maybe_recepetion {
        let response_str = "Failed deserializing ReceptionConfirmation";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Create DeliveryConfirmation
    let confirmation = DeliveryConfirmation {
        accepted_parcel: maybe_recepetion.unwrap().unwrap().reception_response,
        distribution_eh: item.distribution_eh,
        recipient: from,
        recipient_parcel_signature: item.author_signature,
        date_of_response: now(),
    };
    /// Commit DeliveryConfirmation
    let maybe_hh = create_entry(&confirmation);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryConfirmation";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success("DeliveryConfirmation received".to_string());
}