use holo_hash::*;
use holochain::sweettest::{SweetCell, SweetConductor};
use tokio::time::{sleep, Duration};
use zome_delivery_types::{DistributionState, DistributionStrategy, GetNoticeOutput, NoticeState};

use sweettest_utils::*;
use zome_delivery_types::*;
use secret::*;



pub struct SecretAgent {
   cell: SweeterCell,
   signals: Vec<SignalProtocol>,
   ///
   strategy: DistributionStrategy,
}


impl SecretAgent {

   ///
   pub async fn new(conductor: SweetConductor, cell: SweetCell) -> Self {
      Self {
         cell: SweeterCell::new(conductor, cell).await,
         signals: Vec::new(),
         strategy: DistributionStrategy::NORMAL,
      }
   }


   pub fn key(&self) -> AgentPubKey {
      self.cell.key()
   }

   ///
   pub fn set_strategy(&mut self, strategy: DistributionStrategy) {
      self.strategy = strategy;
   }

   pub fn signals(&self) -> Vec<SignalProtocol> {
      self.signals.clone()
   }

   pub async fn drain_signals(&mut self) -> Vec<SignalProtocol> {
      let mut app_signals = self.cell.drain_signals().await;
      //println!("Drained signals count: {}", app_signals.len());
      let new_signals = app_signals.drain(..).map(|app_signal| {
         let signal: SignalProtocol = app_signal.into_inner().decode().unwrap();
         self.signals.push(signal.clone());
         signal.clone()
      }).collect();
      new_signals
   }

   pub async fn print_signals(&self) {
      println!("\n******************** SIGNALS DUMP START ******************** {}", self.key());
      let mut count = 0;
      for signal in self.signals.iter() {
         let signal_txt = print_signal(signal.clone());
         println!(" {:2}. {}", count, signal_txt);
         count += 1;
      }
      println!("******************** SIGNALS DUMP END   ******************** {}\n", count);
   }

   ///
   pub async fn print_chain(&self, millis: u64) {
      sleep(Duration::from_millis(millis)).await;
      self.cell.print_chain().await;
   }

   ///
   pub async fn call_any_zome<I, O>(&self, zome_name: &str, fn_name: &str, payload: I) -> O
      where
         I: serde::Serialize + std::fmt::Debug,
         O: serde::de::DeserializeOwned + std::fmt::Debug,
   {
      return self.cell.call_any_zome(zome_name, fn_name, payload).await;
   }


   ///
   pub async fn call_zome<I, O>(&self, fn_name: &str, payload: I) -> O
   where
      I: serde::Serialize + std::fmt::Debug,
      O: serde::de::DeserializeOwned + std::fmt::Debug,
   {
      return self.cell.call_any_zome("secret", fn_name, payload).await;
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
      return self.cell.try_call_zome(zome_name, fn_name, payload, predicat).await;
   }


   ///
   pub async fn pull_inbox(&self) -> Vec<HeaderHash> {
      return self.call_any_zome("delivery", "pull_inbox", ()).await;
   }


   ///
   pub async fn send(&self, secret_eh: EntryHash, recipient: AgentPubKey) -> EntryHash {
      let input = SendSecretInput {
         secret_eh,
         recipients: vec![recipient],
         strategy: self.strategy.clone(),
      };
      let distribution_eh: EntryHash = self.call_zome("send_secret", input).await;
      return distribution_eh;
   }

   ///
   pub async fn send_multiple(&self, secret_eh: EntryHash, recipients: Vec<AgentPubKey>) -> EntryHash {
      assert!(recipients.len() > 1);
      let input = SendSecretInput {
         secret_eh,
         recipients,
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


   /// -- SIGNALS

   pub fn has_signal(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
      for signal in self.signals.iter() {
         if signal.is(kind, eh) {
            return true;
         }
      }
      false
   }

   ///
   pub async fn pull_and_wait_for_signal(&mut self, kind: SignalKind, eh: &EntryHash) -> Result<(), ()> {
      for _ in 0..10u32 {
         let _ = self.pull_inbox().await;
         let _ = self.drain_signals().await;
         if self.has_signal(&kind, eh) {
            return Ok(())
         };
         tokio::time::sleep(std::time::Duration::from_millis(2 * 1000)).await;
      }
      Err(())
   }
}



fn print_signal(signal: SignalProtocol) -> String {
   match signal {
      SignalProtocol::ReceivedNotice(notice) => {
         format!("ReceivedNotice for {}", notice.distribution_eh)
      },
      SignalProtocol::ReceivedReply(reply) => {
         format!("ReceivedReply for {}", reply.distribution_eh)
      },
      SignalProtocol::ReceivedParcel(parcel) => {
         format!("ReceivedParcel {}", parcel.parcel_eh)
      },
      SignalProtocol::ReceivedReceipt(receipt) => {
         format!("ReceivedReceipt for {}", receipt.distribution_eh)
      },
   }
}