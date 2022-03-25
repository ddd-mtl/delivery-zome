use holo_hash::*;
use tokio::time::{sleep, Duration};
use zome_delivery_types::DistributionStrategy;

use sweettest_utils::*;
use crate::DNA_FILEPATH;
use crate::secret_agent::SecretAgent;
use crate::setup::*;

/// Should fail
pub async fn test_delivery_self() {
   /// Setup
   let (conductor0, alex_key, cell0) = setup_1_conductor(DNA_FILEPATH).await;

   let alex = SecretAgent::new(&conductor0, alex_key, &cell0);

   /// A Store secret
   let secret_eh: EntryHash = alex.call_zome("create_secret", "I like bananas").await;
   println!("secret_eh: {:?}", secret_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret_eh.clone()).await;
   println!("secret_msg: {}", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret to A
   let _distribution_eh: EntryHash = alex.send(secret_eh, alex.key()).await;

   alex.print_chain(2000).await;
}


///
pub async fn test_delivery(strategy: DistributionStrategy) {
   /// Setup
   let (conductors, agents, apps) = setup_2_conductors().await;
   let cells = apps.cells_flattened();

   let mut alex = SecretAgent::new(&conductors[0], agents[0].clone(), cells[0]);
   let mut billy= SecretAgent::new(&conductors[1], agents[1].clone(), cells[1]);
   alex.set_strategy(strategy.clone());
   billy.set_strategy(strategy.clone());

   /// A Store secrets
   let secret_eh: EntryHash = alex.call_zome("create_secret", "I like bananas").await;
   println!("secret_eh: {:?}", secret_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", secret_eh.clone()).await;
   println!("secret_msg: {}", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret to B
   let _ = alex.send(secret_eh, billy.key()).await;

   alex.print_chain(2 * 1000).await;


   /// B checks if Notice received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret","get_secrets_from", agents[0].clone(),
                                                       |result: &Vec<EntryHash>| {result.len() == 1})
      .await
      .expect("Should have a waiting parcel");
   println!("parcel requests received: {}", waiting_parcels.len());

   /// B accepts A's secret
   let _eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;

   billy.print_chain(2 * 1000).await;

   /// Have A receive reply and send Parcel
   sleep(Duration::from_millis(2 * 1000)).await;
   println!("\n A receive reply; pull_inbox()...");
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(2 * 1000).await;

   /// B gets secret
   if strategy.can_dht() {
      println!("\n B trying to get secret pull_inbox()...");
      // let _: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("delivery"), "pull_inbox", ()).await;
      let _: Vec<HeaderHash> = billy.try_call_zome("delivery", "pull_inbox", (),
                                             |result: &Vec<HeaderHash>| { result.len() == 1 })
         .await
         .expect("Should have received 1 parcel");
   }

   billy.print_chain(0).await;

   // let secret: String = try_zome_call_fallible(&conductors[1], &cells[1], "secret", "get_secret", waiting_parcels[0].clone())
   //    .await
   //    .expect("Should have received Secret Parcel");
   println!("\n B calls get_secret()...");
   let secret: String  = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);

   billy.print_chain(0).await;

   /// Check A's chain for a DeliveryReceipt
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;

   alex.print_chain(2 * 1000).await;
}


///
pub async fn test_delivery_manifest(strategy: DistributionStrategy) {
   /// Setup
   let (conductors, agents, apps) = setup_2_conductors().await;
   let cells = apps.cells_flattened();

   let mut alex = SecretAgent::new(&conductors[0], agents[0].clone(), cells[0]);
   let mut billy= SecretAgent::new(&conductors[1], agents[1].clone(), cells[1]);
   alex.set_strategy(strategy.clone());
   billy.set_strategy(strategy.clone());


   /// A Store secret
   let manifest_eh: EntryHash = alex.call_zome("create_split_secret", "I like bananas").await;
   println!("manifest_eh: {:?}", manifest_eh);
   /// A Check secret is stored
   let secret_msg: String = alex.call_zome("get_secret", manifest_eh.clone()).await;
   println!("secret_msg: {}", secret_msg);

   alex.print_chain(200).await;

   /// A sends secret to B
   let _distribution_eh: EntryHash = alex.send(manifest_eh, billy.key()).await;

   sleep(Duration::from_millis(2 * 1000)).await;
   print_chain(&conductors[0], &agents[0], &cells[0]).await;


   /// B checks if request received
   let waiting_parcels: Vec<EntryHash> = billy.try_call_zome("secret", "get_secrets_from", agents[0].clone(),
                                                       |result: &Vec<EntryHash>| {result.len() == 1})
      .await
      .expect("Should have a waiting parcel");
   println!("parcel requests received: {}", waiting_parcels.len());

   /// B accepts A's secret
   let _eh: EntryHash = billy.call_zome("accept_secret", waiting_parcels[0].clone()).await;

   billy.print_chain(2 * 1000).await;

   /// Have A receive reply and send Parcel
   sleep(Duration::from_millis(2 * 1000)).await;
   println!("\n A receive reply; pull_inbox()...");
   let _: Vec<HeaderHash> = alex.pull_inbox().await;
   alex.print_chain(20 * 1000).await;


   /// B gets secret
   if strategy.can_dht() {
      println!("\n B trying to get secret pull_inbox()...");
      // let _: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("delivery"), "pull_inbox", ()).await;
      let _: Vec<HeaderHash> = billy.try_call_zome("delivery", "pull_inbox", (),
                                             |result: &Vec<HeaderHash>| { result.len() == 4 })
         .await
         .expect("Should have received 1 parcel");
   }
   billy.print_chain(5 * 1000).await;

   println!("\n B calls get_secret()...");
   let secret: String = billy.call_zome("get_secret", waiting_parcels[0].clone()).await;
   println!("\n secret received: {:?}\n", secret);

   billy.print_chain(0).await;


   /// Check A's chain for a DeliveryReceipt
   sleep(Duration::from_millis(2 * 1000)).await;
   let _: Vec<HeaderHash> = alex.pull_inbox().await;

   alex.print_chain(2 * 1000).await;

}
