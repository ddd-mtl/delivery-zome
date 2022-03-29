use holo_hash::*;
use zome_delivery_types::{DistributionState, DistributionStrategy, NoticeState, SignalKind};

use crate::setup::*;

/// A sends secret 1 to B
/// A sends secret 2 to B & C
/// B sends secret 4 to A
/// A sends secret 3 to B
pub async fn test_multiple_delivery(strategy: DistributionStrategy) {
   /// Setup
   let (mut alex, mut billy, mut camille) = setup_3_secret_agents(strategy.clone()).await;
   /// Store secrets
   let secret1_eh: EntryHash = alex.call_zome("create_secret", "A like bananas").await;
   let secret2_eh: EntryHash = alex.call_zome("create_secret", "A hates apples").await;
   let secret3_eh: EntryHash = alex.call_zome("create_secret", "A eats fruits").await;
   let secret4_eh: EntryHash = billy.call_zome("create_secret", "B eats cereals").await;
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret1_eh.clone()).await;
   println!("secret_msg: {}\n", secret_msg);
   alex.print_chain(200).await;

   /// A sends secrets
   let distribution1_eh: EntryHash = alex.send(secret1_eh.clone(), billy.key()).await;
   let distribution2_eh: EntryHash = alex.send_multiple(secret2_eh.clone(), vec![billy.key(), camille.key()]).await;
   /// B sends secret 4 to A
   let distribution4_eh: EntryHash = billy.send(secret4_eh.clone(), alex.key()).await;

   /// Check if all notices' have been received
   alex.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution4_eh).await.expect("Should have received notice");
   alex.print_chain(0).await;
   billy.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution1_eh).await.expect("Should have received notice");
   billy.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution2_eh).await.expect("Should have received notice");
   billy.print_chain(0).await;
   //billy.print_signals().await;
   camille.print_chain(0).await;
   camille.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution2_eh).await.expect("Should have received notice");
   camille.print_chain(0).await;

   /// B checks if Notices have been received
   let billy_waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", alex.key(),
                                                       |result: &Vec<EntryHash>| {result.len() == 2})
      .await
      .expect("Billy should have two waiting parcels");
   println!("Billy. Parcel requests received: {}\n", billy_waiting_parcels.len());
   /// B accepts A's secret 1 and refuse secret 2
   let _eh: EntryHash = billy.call_zome("accept_secret", billy_waiting_parcels[0].clone()).await;
   let _eh: EntryHash = billy.call_zome("refuse_secret", billy_waiting_parcels[1].clone()).await;
   billy.print_chain(6 * 1000).await;

   /// C checks if notice 2 received
   let camille_waiting_parcels: Vec<EntryHash> = camille.try_call_zome("secret","get_secrets_from", alex.key(),
                                                             |result: &Vec<EntryHash>| {result.len() == 1})
                                              .await
                                              .expect("Camille should have a waiting parcel");
   println!("Camille. Parcel requests received: {}\n", camille_waiting_parcels.len());
   /// C accepts A's secret 2
   let _eh: EntryHash = camille.call_zome("accept_secret", camille_waiting_parcels[0].clone()).await;
   camille.print_chain(6 * 1000).await;

   /// Have A receive reply and send Parcel
   println!("\n A receive reply; pull_inbox()...\n");
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution1_eh).await.expect("Should have received reply");
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution2_eh).await.expect("Should have received reply");
   alex.print_chain(0).await;

   /// A sends secret 3 to B
   let distribution3_eh: EntryHash = alex.send(secret3_eh.clone(), billy.key()).await;
   /// A accepts B's secret 4
   let alex_waiting_parcels: Vec<EntryHash> = alex.try_call_zome("secret","get_secrets_from", billy.key(),
                                                             |result: &Vec<EntryHash>| {result.len() == 1})
                                              .await
                                              .expect("Alex should have a waiting parcel");
   println!("Alex. Parcel requests received: {}\n", alex_waiting_parcels.len());
   let _eh: EntryHash = alex.call_zome("accept_secret", alex_waiting_parcels[0].clone()).await;


   /// Have B receive parcel 1 and notice 3
   billy.pull_and_wait_for_signal(SignalKind::ReceivedParcel, &secret1_eh).await.expect("Should have received parcel");
   billy.pull_and_wait_for_signal(SignalKind::ReceivedNotice, &distribution3_eh).await.expect("Should have received notice");
   billy.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution4_eh).await.expect("Should have received reply");
   billy.print_chain(0).await;
   println!("\n B calls get_secret()...\n");
   let secret: String  = billy.call_zome("get_secret", billy_waiting_parcels[0].clone()).await;
   println!("\n B received secret 1: {:?}\n", secret);
   billy.assert_notice_state(distribution1_eh.clone(), NoticeState::Received).await;
   billy.print_chain(0).await;

   /// Have C receive parcel 2
   camille.pull_and_wait_for_signal(SignalKind::ReceivedParcel, &secret2_eh).await.expect("Should have received parcel");
   let secret: String  = camille.call_zome("get_secret", camille_waiting_parcels[0].clone()).await;
   println!("\n C received secret 2: {:?}\n", secret);
   camille.assert_notice_state(distribution2_eh.clone(), NoticeState::Received).await;
   camille.print_chain(0).await;

   /// Check A's chain for a DeliveryReceipt
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReceipt, &distribution1_eh).await.expect("Should have received receipt");
   alex.assert_distribution_state(distribution1_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;
   alex.assert_distribution_state(distribution2_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;
   alex.print_chain(0).await;


   /// B checks if notice 3 received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", alex.key(),
                                                             |result: &Vec<EntryHash>| {result.len() == 3})
                                              .await
                                              .expect("Billy should have a waiting parcel");
   println!("billy parcel requests received: {}\n", waiting_parcels.len());

   /// B accepts A's secret 3
   let _eh: EntryHash = billy.call_zome("accept_secret", secret3_eh.clone()).await;
   billy.print_chain(2 * 1000).await;

   /// Have A receive reply and send Parcel 3
   println!("\n A receive reply for secret 3; pull_inbox()...\n");
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReply, &distribution3_eh).await.expect("Should have received reply");
   alex.print_chain(0).await;

   /// B gets A's secret 3
   billy.pull_and_wait_for_signal(SignalKind::ReceivedParcel, &secret3_eh).await.expect("Should have received parcel");
   billy.print_chain(0).await;
   println!("\n B calls get_secret()...\n");
   let secret: String  = billy.call_zome("get_secret", secret3_eh.clone()).await;
   println!("\n B received secret 3: {:?}\n", secret);
   billy.assert_notice_state(distribution3_eh.clone(), NoticeState::Received).await;

   /// A gets B's secret 4
   alex.pull_and_wait_for_signal(SignalKind::ReceivedParcel, &secret4_eh).await.expect("Should have received parcel");
   alex.print_chain(0).await;
   println!("\n A calls get_secret()...\n");
   let secret: String  = alex.call_zome("get_secret", secret4_eh.clone()).await;
   println!("\n A received secret 4: {:?}\n", secret);
   alex.assert_notice_state(distribution4_eh.clone(), NoticeState::Received).await;

   /// Check A's chain for a DeliveryReceipt
   alex.pull_and_wait_for_signal(SignalKind::ReceivedReceipt, &distribution3_eh).await.expect("Should have received receipt");
   alex.print_chain(0).await;
   alex.print_signals().await;
   alex.assert_distribution_state(distribution3_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;

   /// Check B's chain for a DeliveryReceipt
   billy.pull_and_wait_for_signal(SignalKind::ReceivedReceipt, &distribution4_eh).await.expect("Should have received receipt");
   billy.print_chain(0).await;
   billy.print_signals().await;
   billy.assert_distribution_state(distribution4_eh.clone(), DistributionState::AllAcceptedParcelsReceived).await;
}