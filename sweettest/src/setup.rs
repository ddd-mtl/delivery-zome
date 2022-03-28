use holochain_zome_types::X25519PubKey;
use holochain::sweettest::*;
use holo_hash::*;
use tokio::time::{sleep, Duration};
use sweettest_utils::*;
use zome_delivery_types::DistributionStrategy;

use crate::DNA_FILEPATH;
use crate::secret_agent::SecretAgent;


///
pub async fn setup_2_secret_agents(strategy: DistributionStrategy) -> (SecretAgent, SecretAgent) {
   let (conductors, _agents, apps) = setup_2_conductors().await;
   let mut apps = apps.into_inner();
   let mut conductor_vec = conductors.into_inner();

   let mut alex = SecretAgent::new(
      conductor_vec.pop().unwrap(),
      apps.pop().unwrap().into_cells().pop().unwrap())
      .await;
   let mut billy = SecretAgent::new(
      conductor_vec.pop().unwrap(),
      apps.pop().unwrap().into_cells().pop().unwrap())
      .await;

   alex.set_strategy(strategy.clone());
   billy.set_strategy(strategy.clone());

   return (alex, billy);
}


pub async fn setup_3_secret_agents(strategy: DistributionStrategy) -> (SecretAgent, SecretAgent, SecretAgent) {
   let (conductors, _agents, apps) = setup_3_conductors().await;
   let mut apps = apps.into_inner();
   let mut conductor_vec = conductors.into_inner();

   let mut alex = SecretAgent::new(conductor_vec.pop().unwrap(), apps.pop().unwrap().into_cells().pop().unwrap()).await;
   let mut billy = SecretAgent::new(conductor_vec.pop().unwrap(), apps.pop().unwrap().into_cells().pop().unwrap()).await;
   let mut camille = SecretAgent::new(conductor_vec.pop().unwrap(), apps.pop().unwrap().into_cells().pop().unwrap()).await;


   alex.set_strategy(strategy.clone());
   billy.set_strategy(strategy.clone());
   camille.set_strategy(strategy.clone());

   return (alex, billy, camille);
}


///
pub async fn setup_2_conductors() -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let (conductors, agents, apps) = setup_conductors(DNA_FILEPATH, 2).await;
   let cells = apps.cells_flattened();

   println!("* WAITING FOR INIT TO FINISH...\n\n");
   sleep(Duration::from_millis(5 * 1000)).await;

   println!("\n\n\n CALLING get_enc_key() TO SELF ...\n\n");
   let _: X25519PubKey = try_zome_call_fallible(&conductors[0], &cells[0],"delivery", "get_enc_key", &agents[0])
      .await.unwrap();
   let _: X25519PubKey = try_zome_call_fallible(&conductors[1], &cells[1],"delivery", "get_enc_key", &agents[1])
      .await.unwrap();
   println!("\n\n\n CALLING get_enc_key() TO FRIEND ...\n\n");
   let _: X25519PubKey = try_zome_call_fallible(&conductors[1], &cells[1],"delivery", "get_enc_key", &agents[0])
      .await.unwrap();
   println!("\n\n\n AGENTS SETUP DONE\n\n");

   print_peers(&conductors[1], &cells[1]).await;

   (conductors, agents, apps)
}


///
pub async fn setup_3_conductors() -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let (conductors, agents, apps) = setup_conductors(DNA_FILEPATH, 3).await;
   let cells = apps.cells_flattened();

   println!("\n\n\n WAITING FOR INIT TO FINISH...\n\n");
   sleep(Duration::from_millis(10 * 1000)).await;

   let _: X25519PubKey = try_zome_call_fallible(&conductors[0], &cells[0],"delivery", "get_enc_key", &agents[0])
      .await.unwrap();
   let _: X25519PubKey = try_zome_call_fallible(&conductors[1], &cells[1],"delivery", "get_enc_key", &agents[1])
      .await.unwrap();
   //let _: X25519PubKey = conductors[1].call(&cells[1].zome("delivery"), "get_enc_key", &agents[1]).await;

   println!("\n\n\n AGENTS SETUP DONE\n\n");

   (conductors, agents, apps)
}
