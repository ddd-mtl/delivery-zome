use holo_hash::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use tokio::time::{sleep, Duration};
use zome_delivery_types::DistributionStrategy;

use sweettest_utils::*;

use secret::*;


pub struct SecretAgent<'a> {
   agent: AgentPubKey,
   cell: &'a SweetCell,
   conductor: &'a SweetConductor,
   strategy: DistributionStrategy,
}


impl<'a> SecretAgent<'a> {

   ///
   pub fn new(conductor: &'a SweetConductor, agent: AgentPubKey, cell:  &'a SweetCell) -> Self {
      Self {
         agent,
         cell,
         conductor,
         strategy: DistributionStrategy::NORMAL,
      }
   }


   pub fn key(&self) -> AgentPubKey {
      self.agent.clone()
   }


   ///
   pub fn set_strategy(&mut self, strategy: DistributionStrategy) {
      self.strategy = strategy;
   }


   ///
   pub async fn print_chain(&self, millis: u64) {
      sleep(Duration::from_millis(millis)).await;
      print_chain(self.conductor, &self.agent, self.cell).await;
   }


   ///
   pub async fn call_zome<I, O>(&self, fn_name: &str, payload: I) -> O
   where
      I: serde::Serialize + std::fmt::Debug,
      O: serde::de::DeserializeOwned + std::fmt::Debug,
   {
      return self.conductor.call(&self.cell.zome("secret"), fn_name, payload).await;
   }


   ///
   pub async fn try_call_zome<P, T>(
      &self,
      zome_name: &str,
      fn_name: &str,
      payload: P,
      predicat: fn(res: &T) -> bool,
   ) -> Result<T, ()>
      where
         T: serde::de::DeserializeOwned + std::fmt::Debug,
         P: Clone + serde::Serialize + std::fmt::Debug,
   {
      for _ in 0..10u32 {
         let res: T = self.conductor.call(&self.cell.zome(zome_name), fn_name, payload.clone())
            .await;
         if predicat(&res) {
            return Ok(res);
         }
         tokio::time::sleep(std::time::Duration::from_millis(2 * 1000)).await;
      }
      Err(())
   }


   ///
   pub async fn pull_inbox(&self) -> Vec<HeaderHash> {
      return self.conductor.call(&self.cell.zome("delivery"), "pull_inbox", ()).await;
   }


   ///
   pub async fn send(&self, secret_eh: EntryHash, recipient: AgentPubKey) -> EntryHash {
      let input = SendSecretInput {
         secret_eh,
         recipient,
         strategy: self.strategy.clone(),
      };
      let distribution_eh: EntryHash = self.call_zome("send_secret", input).await;
      return distribution_eh;
   }
}