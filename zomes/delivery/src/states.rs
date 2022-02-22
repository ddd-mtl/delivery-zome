use hdk::prelude::*;


/// State of a single delivery of a mail or ack to a unique recipient
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeliveryState {
   /// Initial state ; Distribution Entry committed
   Unsent,
   /// There is a link to a PendingItem entry holding the DeliveryNotice and the inbox link is alive
   PendingNotice,
   /// NoticeDelivered committed (DeleteLink entry found)
   NoticeDelivered,
   /// Positive ReplyReceived committed
   ParcelRefused,
   /// Negative ReplyReceived committed
   ParcelAccepted,
   /// There is a link to a PendingItem entry holding the Parcel and inbox link is alive
   PendingParcel,
   /// DeliveryReceipt committed (DeleteLink entry found)
   ParcelDelivered,
}

/// Possible states of an OutMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DistributionState {
   /// Initial state ; Distribution committed
   Unsent,
   /// (yellow) All deliveries have been sent (no Unsent state)
   AllSent,
   /// (black) All notices have been sent (no Unsent or PendingNotice state)
   AllNoticeReceived,
   /// (blue) All deliveries have ParcelRefused, ParcelAccepted or PendingParcel state
   AllRepliesReceived,
   /// (green) All deliveries have ParcelRefused or ParcelDelivered state
   AllAcceptedParcelsReceived,
   /// (red) Delete entry commited
   Deleted,
}

/// Possible states of a DeliveryNotice entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NoticeState {
   /// DeliveryNotice committed
   Unreplied,
   /// ReceptionConfirmation(yes) committed
   Accepted,
   /// ReceptionConfirmation(no) committed
   Refused,
   /// ParcelReceived committed
   Received,
   /// ??? Parcel deleted ???
   Deleted,
}
