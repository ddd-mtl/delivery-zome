use hdk::prelude::*;

use crate::{
    utils::*,
            //send_item::*,
            parcel::*, EntryKind,
};
//use crate::entries::*;
//use crate::entries::pub_enc_key::*;


/// Entry representing a received Manifest
#[hdk_entry(id = "DeliveryNotice", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryNotice {
    pub distribution_eh: EntryHash,
    pub sender: AgentPubKey,
    pub sender_summary_signature: Signature,
    pub parcel_summary: ParcelSummary,
}

pub enum DeliveryNoticeQueryField {
    Sender(AgentPubKey),
    Parcel(EntryHash)
}

impl DeliveryNotice {

    ///Find DeliveryNotice with field with given value
    pub fn query(query_field: DeliveryNoticeQueryField) -> ExternResult<Vec<DeliveryNotice>> {
        /// Get all Create ParcelReceived Elements with query
        let query_args = ChainQueryFilter::default()
           .include_entries(true)
           .header_type(HeaderType::Create)
           .entry_type(EntryKind::DeliveryNotice.as_type());
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
                    if notice.parcel_summary.reference.entry_address() == parcel_eh {
                        res.push(notice.clone());
                    }
                }
            }
        }
        /// Done
        Ok(res)
    }


    ///
    pub(crate) fn post_commit(notice_eh: &EntryHash, _notice: Self) -> ExternResult<()> {
        debug!("post_commit_delivery_notice() {:?}", notice_eh);

        // /// Emit signal
        // let item = MailItem {
        //     hh: maybe_hh.unwrap(),
        //     author: from.clone(),
        //     mail: msg.description.clone(),
        //     state: MailState::In(IncomingDeliveryState::ManifestReceived),
        //     bcc: Vec::new(),
        //     date: snapmail_now() as i64, // FIXME
        // };
        // let res = emit_signal(&SignalProtocol::ReceivedReceptionRequest(item));
        // if let Err(err) = res {
        //     error!("Emit signal failed: {}", err);
        // }

        // FIXME delete pending item link


        /// Done
        Ok(())
    }
}



