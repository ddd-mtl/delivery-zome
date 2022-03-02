use hdk::prelude::*;
use zome_delivery_types::*;

entry_defs![
   /// -- PubEncKey
   PubEncKey::entry_def(),
   /// -- Delivery
   DeliveryNotice::entry_def(),
   DeliveryReceipt::entry_def(),
   DeliveryReply::entry_def(),
   Distribution::entry_def(),
   NoticeReceived::entry_def(),
   ParcelReceived::entry_def(),
   ReplyReceived::entry_def(),
   PendingItem::entry_def(),
   ParcelManifest::entry_def(),
   ParcelChunk::entry_def(),
   /// -- Other
   PathEntry::entry_def()
];
