use holo_hash::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use holochain_zome_types::AppSignal;
use tokio::time::{sleep, Duration};
use zome_delivery_types::{DistributionState, DistributionStrategy, GetNoticeOutput, NoticeState};

use sweettest_utils::*;

use secret::*;


pub struct SecretAgent {
   agent: AgentPubKey,
   cell: SweetCell,
   conductor: SweetConductor,
   strategy: DistributionStrategy,
   signal_stack: Vec<AppSignal>,
}


impl SecretAgent {

   ///
   pub fn new(conductor: SweetConductor, agent: AgentPubKey, cell: SweetCell) -> Self {

      // // Make the channel buffer big enough to not block
      // let (resp, mut recv) = tokio::sync::mpsc::channel(NUM_CONDUCTORS * NUM_MESSAGES * 2);
      // let mut jhs = Vec::new();
      // let (trigger, valve) = Valve::new();
      // let total_recv = Arc::new(AtomicUsize::new(0));
      // for c in conductors.iter_mut() {
      //    use futures::stream::StreamExt;
      //    let mut stream = valve.wrap(c.signals().await);
      //    let jh = tokio::task::spawn({
      //       let mut resp = resp.clone();
      //       let total_recv = total_recv.clone();
      //       async move {
      //          while let Some(Signal::App(_, signal)) = stream.next().await {
      //             let signal: SignalPayload = signal.into_inner().decode().unwrap();
      //             if let SignalPayload::Message(SignalMessageData {
      //                                              message_data:
      //                                              MessageData {
      //                                                 message: Message { uuid, .. },
      //                                                 ..
      //                                              },
      //                                              ..
      //                                           }) = signal
      //             {
      //                total_recv.fetch_add(1, Ordering::Relaxed);
      //                resp.send(uuid).await.expect("Failed to send uuid");
      //             }
      //          }
      //       }
      //    });
      //    jhs.push(jh);
      // }


      Self {
         agent,
         cell,
         conductor,
         strategy: DistributionStrategy::NORMAL,
         signal_stack: Vec::new(),
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