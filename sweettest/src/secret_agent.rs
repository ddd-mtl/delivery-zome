use holo_hash::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use tokio::time::{sleep, Duration};
use zome_delivery_types::{DistributionState, DistributionStrategy, GetNoticeOutput, NoticeState};


use std::sync::Arc;
use stream_cancel::{Trigger, Valve};

use sweettest_utils::*;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use secret::*;
use zome_delivery_types::*;
use holochain_types::prelude::*;


pub struct SecretAgent {
   agent: AgentPubKey,
   cell: SweetCell,
   conductor: SweetConductor,
   strategy: DistributionStrategy,
   // Signal handling
   signals: Arc<Mutex<Vec<SignalProtocol>>>,
   jh: JoinHandle<()>,
   trigger: Trigger,
}


impl SecretAgent {

   ///
   pub async fn new(mut conductor: SweetConductor, agent: AgentPubKey, cell: SweetCell) -> Self {

      let signals = Arc::new(Mutex::new(vec![]));

      let (trigger, valve) = Valve::new();

      use futures::stream::StreamExt;
      let mut stream = valve.wrap(conductor.signals());
      let jh = tokio::task::spawn({
         let clone = Arc::clone(&signals);
         async move {
            while let Some(Signal::App(_, app_signal)) = stream.next().await {
               let signal: SignalProtocol = app_signal.into_inner().decode().unwrap();
               println!("\n SIGNAL RECEIVED: {:?}\n\n", signal);
               let mut v = clone.lock().await;
               v.push(signal);
            }
         }
      });


      // /// Drop
      // trigger.cancel();
      // for jh in jhs {
      //    jh.await.unwrap();
      // }

      Self {
         agent,
         cell,
         conductor,
         strategy: DistributionStrategy::NORMAL,
         signals,
         jh,
         trigger,
      }
   }


   pub fn key(&self) -> AgentPubKey {
      self.agent.clone()
   }

   ///
   pub fn set_strategy(&mut self, strategy: DistributionStrategy) {
      self.strategy = strategy;
   }

   pub async fn signals(&self) -> Vec<SignalProtocol> {
      self.signals.lock().await.clone()
   }

   pub async fn print_signals(&self) {
      println!("\n****** SIGNALS DUMP START ****** {}", self.agent);
      let signals = self.signals.lock().await.clone();
      let mut count = 0;
      for signal in signals {
         println!(" {:2}. {:?}", count, signal);
         count += 1;
      }
      println!("\n****** SIGNALS DUMP END   ****** {}", count);
   }

   ///
   pub async fn print_chain(&self, millis: u64) {
      sleep(Duration::from_millis(millis)).await;
      print_chain(&self.conductor, &self.agent, &self.cell).await;
   }

   ///
   pub async fn call_any_zome<I, O>(&self, zome_name: &str, fn_name: &str, payload: I) -> O
      where
         I: serde::Serialize + std::fmt::Debug,
         O: serde::de::DeserializeOwned + std::fmt::Debug,
   {
      return self.conductor.call(&self.cell.zome(zome_name), fn_name, payload).await;
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


   ///
   pub async fn assert_notice_state(&self, distribution_eh: EntryHash, required_state: NoticeState) {
      // Make sure distribution is from this agent
      let maybe_output: Option<GetNoticeOutput> = self.call_any_zome("delivery", "get_notice", distribution_eh.clone())
                                                       .await;
      let notice_state = maybe_output.unwrap().state;
      //println!("Notice state: {:?}", notice_state);
      assert_eq!(notice_state, required_state);
   }

   ///
   pub async fn assert_distribution_state(&self, distribution_eh: EntryHash, required_state: DistributionState) {
      let state: DistributionState = self.call_any_zome("delivery", "get_distribution_state", distribution_eh).await;
      //println!("Distribution state: {:?}", state);
      assert_eq!(state, required_state);
   }
}