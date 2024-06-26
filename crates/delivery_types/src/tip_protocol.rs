use std::fmt;
use hdi::prelude::*;
use crate::*;


///  Protocol for sending data between agents
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DeliveryTipProtocol {
  PublicParcelPublished((EntryHash, Timestamp, ParcelReference)),
  PublicParcelUnpublished((EntryHash, Timestamp, ParcelReference)),
}
impl fmt::Display for DeliveryTipProtocol {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let str: String = match self {
      DeliveryTipProtocol::PublicParcelPublished((_pr_eh, _ts, _pr, )) => format!("PublicParcel Published"),
      DeliveryTipProtocol::PublicParcelUnpublished((_pr_eh, _ts, _pr, )) => format!("PublicParcel Unpublished"),
    };
    fmt.write_str(&str)?;
    Ok(())
  }
}
