use holo_hash::*;
use zome_delivery_types::*;

use sweettest_utils::*;
use crate::DNA_FILEPATH;
use crate::secret_agent::SecretAgent;
use crate::setup::*;

/// Should fail
pub async fn test_delivery_self() {
   /// Setup
   let (conductor0, _alex_key, cell0) = setup_1_conductor(DNA_FILEPATH).await;

   let alex = SecretAgent::new(conductor0, cell0).await;

   /// A Store secret
   let secret_eh: EntryHash = alex.call_zome("create_secret", "I like bananas").await;
   println!("secret_eh: {:?}\n", secret_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret_eh.clone()).await;
   println!("secret_msg: {}\n", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret to A
   let distribution_eh: EntryHash = alex.send(secret_eh, alex.key()).await;

   alex.print_chain(2000).await;

   alex.assert_distribution_state(distribution_eh, DistributionState::Unsent).await;

}



///
pub async fn test_delivery(strategy: DistributionStrategy) {
   /// Setup
   let (mut alex, mut billy) = setup_2_secret_agents(strategy.clone()).await;
   /// A Store secrets
   let secret_eh: EntryHash = alex.call_zome("create_secret", "I like bananas").await;
   println!("secret_eh: {:?}\n", secret_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret_eh.clone()).await;
   println!("secret_msg: {}\n", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret to B
   let distribution_eh = alex.send(secret_eh.clone(), billy.key()).await;
   alex.print_chain(2 * 1000).await;

   let state: DistributionState = alex.call_any_zome("delivery", "get_distribution_state", distribution_eh.clone()).await;
   println!("Distribution state: {:?}\n", state);
   //assert_eq!(DistributionState::AllNoticeReceived, state);


   /// B checks if Notice received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", alex.key(),
                                                       |result: &Vec<EntryHash>| {result.len() == 1})
      .await;
   println!("parcel requests received: {}\n", waiting_parcels.len());
   billy.drain_signals().await;
   assert!(billy.has_signal(&SignalKind::ReceivedNotice, &distribution_eh));
   billy.assert_notice_state(distribution_eh.clone(), NoticeState::Unreplied).await;


   /// B accepts A's secret
   let _reply_eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;
   billy.print_chain(2 * 1000).await;
   //billy.assert_notice_state(distribution_eh.clone(), NoticeState::Accepted or NoticeState::Received).await;

   /// Have A receive reply and send Parcel
   println!("\n A receive reply; pull_inbox()...\n");
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution_eh).await.expect("Should have received reply");
   alex.print_chain(0).await;
   let state: DistributionState = alex.call_any_zome("delivery", "get_distribution_state", distribution_eh.clone()).await;
   println!("Distribution state: {:?}", state);
   //assert_eq!(DistributionState::AllNoticeReceived, state);

   /// B gets secret
   if strategy.can_dht() {
      println!("\n B trying to get secret pull_inbox()...\n");
      // let _: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("delivery"), "pull_inbox", ()).await;
      let _: Vec<HeaderHash> = billy.try_call_zome("delivery", "pull_inbox", (),
                                             |result: &Vec<HeaderHash>| { result.len() == 1 })
         .await;
   }
   billy.drain_signals().await;
   assert!(billy.has_signal(&SignalKind::ReceivedParcel, &secret_eh));
   billy.print_chain(0).await;

   println!("\n B calls get_secret()...\n");
   let secret: String  = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);
   billy.print_chain(1 * 1000).await;
   billy.assert_notice_state(distribution_eh.clone(), NoticeState::Received).await;
   billy.drain_signals().await;
   billy.print_signals().await;

   /// Check A's chain for a DeliveryReceipt
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReceipt, &distribution_eh).await.expect("Should have received receipt");
   alex.print_chain(0).await;
   alex.print_signals().await;
   alex.assert_distribution_state(distribution_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;
}


///
pub async fn test_delivery_manifest(strategy: DistributionStrategy) {
   /// Setup
   let (mut alex, mut billy) = setup_2_secret_agents(strategy.clone()).await;

   /// A Store secret
   let manifest_eh: EntryHash = alex.call_zome("create_split_secret", "I like bananas").await;
   println!("manifest_eh: {:?}\n", manifest_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", manifest_eh.clone()).await;
   println!("secret_msg: {}\n", secret_msg);
   alex.print_chain(200).await;

   /// A sends secret to B
   let distribution_eh: EntryHash = alex.send(manifest_eh.clone(), billy.key()).await;
   billy.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution_eh).await.expect("Should have received notice");
   // alex.print_chain(10 * 1000).await;

   /// B checks if request received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret", "get_secrets_from", alex.key(),
                                                       |result: &Vec<EntryHash>| {result.len() == 1})
      .await;
   println!("parcel requests received: {}\n", waiting_parcels.len());
   billy.assert_notice_state(distribution_eh.clone(), NoticeState::Unreplied).await;


   /// B accepts A's secret
   let _eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;
   billy.assert_notice_state(distribution_eh.clone(), NoticeState::Accepted).await;
   billy.print_chain(10 * 1000).await;

   /// Have A receive reply and send Parcel
   println!("\n A receive reply; pull_inbox()...\n");
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution_eh).await.expect("Should have received reply");
   //alex.assert_distribution_state(distribution_eh.clone(), DistributionState::AllRepliesReceived).await;

   /// Have B receive parcel
   billy.pull_and_wait_for_signal(SignalKind::ReceivedParcel, &manifest_eh).await.expect("Should have received parcel");
   billy.print_chain(0).await;

   println!("\n B calls get_secret()...\n");
   let secret: String = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);
   //assert_eq!(secret, "I.like.bananas");
   billy.assert_notice_state(distribution_eh.clone(), NoticeState::Received).await;
   billy.drain_signals().await;
   billy.print_signals().await;

   /// Check A's chain for a DeliveryReceipt
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReceipt, &distribution_eh).await.expect("Should have received receipt");
   alex.print_chain(0).await;
   alex.print_signals().await;
   alex.assert_distribution_state(distribution_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;


}
