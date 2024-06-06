use std::fmt;
use hdi::prelude::*;
use crate::*;


///  Protocol for sending data between agents
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryGossipProtocol {
  PublicParcelPublished((EntryHash, Timestamp, ParcelReference)),
  PublicParcelUnpublished((EntryHash, Timestamp, ParcelReference)),
  Ping,
  Pong,
}
impl fmt::Display for DeliveryGossipProtocol {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let str: String = match self {
      DeliveryGossipProtocol::PublicParcelPublished((_pr_eh, _ts, _pr, )) => format!("PublicParcel Published"),
      DeliveryGossipProtocol::PublicParcelUnpublished((_pr_eh, _ts, _pr, )) => format!("PublicParcel Unpublished"),
      DeliveryGossipProtocol::Ping => "Ping".to_owned(),
      DeliveryGossipProtocol::Pong => "Pong".to_owned(),
    };
    fmt.write_str(&str)?;
    Ok(())
  }
}
