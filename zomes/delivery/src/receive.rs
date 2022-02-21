use hdk::prelude::*;

use crate::{
    DeliveryProtocol,
    ItemMessage,
    signal_protocol::*,
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
                PendingKind::ParcelDescription => receive_dm_description(from, item),
                PendingKind::DeliveryNotification  => receive_dm_notification(from, item),
                PendingKind::Entry => {/* FIXME */},
                PendingKind::ReceptionConfirmation => receive_dm_reception(from, item),
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
pub fn receive_dm_description(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_description: ExternResult<Option<ParcelDescription>> = item.try_into(from.clone());
    if let Err(err) = maybe_description {
        let response_str = "Failed deserializing ParcelDescription";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Create DeliveryNotification
    let notification = DeliveryNotification {
        description: maybe_description.unwrap().unwrap(), // Should be ParcelDescription since we filtered before calling this function
        sender: from,
        sender_description_signature: item.author_signature,
        sender_distribution_eh: item.distribution_eh,
    };
    /// Commit DeliveryNotification
    let maybe_hh = create_entry(&notification);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryNotification";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success("ParcelDescription received".to_string());
}


/// Returns Success or Failure
pub fn receive_dm_notification(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
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