use holo_hash::*;
use tokio::time::{sleep, Duration};
use zome_delivery_types::DistributionStrategy;

use crate::setup::*;

///
pub async fn test_multiple_delivery(strategy: DistributionStrategy) {
   /// Setup
   let (alex, billy, camille) = setup_3_secret_agents(strategy.clone()).await;

   /// A Store secrets
   let secret1_eh: EntryHash = alex.call_zome("create_secret", "I like bananas").await;
   let secret2_eh: EntryHash = alex.call_zome("create_secret", "You hate apples").await;
   let secret3_eh: EntryHash = alex.call_zome("create_secret", "They eat fruits").await;
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret1_eh.clone()).await;
   println!("secret_msg: {}", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret 1 & 2 to B
   let _ = alex.send(secret1_eh, billy.key()).await;
   let _ = alex.send(secret2_eh, billy.key()).await;

   alex.print_chain(200).await;

   /// B checks if Notice received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", alex.key(),
                                                       |result: &Vec<EntryHash>| {result.len() == 2})
      .await
      .expect("Should have a waiting parcel");
   println!("parcel requests received: {}", waiting_parcels.len());

   /// B accepts A's secret 1 & 2
   let _eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;
   let _eh: EntryHash = billy.call_zome("refuse_secret", waiting_parcels[1].clone()).await;

   billy.print_chain(2 * 1000).await;

   /// Have A receive reply and send Parcel
   println!("\n A receive reply; pull_inbox()...");
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(2 * 1000).await;

   /// A sends secret 3 to B
   let _ = alex.send(secret3_eh, billy.key()).await;

   /// B gets secret
   if strategy.can_dht() {
      println!("\n B trying to get secret pull_inbox()...");
      let _: Vec<HeaderHash> = billy.try_call_zome("delivery", "pull_inbox", (),
                                             |result: &Vec<HeaderHash>| { result.len() == 2 })
         .await
         .expect("Should have received 1 parcel");
   }
   //sleep(Duration::from_millis(2 * 1000)).await;
   billy.print_chain(0).await;

   println!("\n B calls get_secret()...");
   let secret: String  = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);
   let secret: String  = billy.call_zome("get_secret", waiting_parcels[1].clone()).await;
   println!("\n secret received: {:?}\n", secret);
   billy.print_chain(0).await;

   /// Check A's chain for a DeliveryReceipt
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(2 * 1000).await;


   /// B checks if Notice received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", alex.key(),
                                                             |result: &Vec<EntryHash>| {result.len() == 1})
                                              .await
                                              .expect("Should have a waiting parcel");
   println!("parcel requests received: {}", waiting_parcels.len());

   /// B accepts A's secret 3
   let _eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;

   billy.print_chain(2 * 1000).await;

   /// Have A receive reply and send Parcel 3
   println!("\n A receive reply; pull_inbox()...");
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(2 * 1000).await;



   /// B gets secret
   if strategy.can_dht() {
      println!("\n B trying to get secret pull_inbox()...");
      let _: Vec<HeaderHash> = billy.try_call_zome("delivery", "pull_inbox", (),
                                                   |result: &Vec<HeaderHash>| { result.len() == 1 })
                                    .await
                                    .expect("Should have received 1 parcel");
   }
   //sleep(Duration::from_millis(2 * 1000)).await;
   billy.print_chain(0).await;

   println!("\n B calls get_secret()...");
   let secret: String  = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);

   /// Check A's chain for a DeliveryReceipt
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(2 * 1000).await;
}