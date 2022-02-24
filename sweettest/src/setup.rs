
use holochain::conductor::*;
use holochain::sweettest::*;
use holochain_state::source_chain::*;
use holochain_zome_types::*;
use holochain::conductor::config::ConductorConfig;
use holo_hash::*;
use holochain_p2p::*;
use colored::*;
use futures::future;

pub const DNA_FILEPATH: &str = "./secret.dna";
pub const ALEX_NICK: &str = "alex";
pub const BILLY_NICK: &str = "billy";
pub const CAMILLE_NICK: &str = "camille";


///
pub fn create_network_config() -> ConductorConfig {
   std::env::set_var("KIT_PROXY", "kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--");
   let kitsune_config = SweetNetwork::env_var_proxy()
      .expect("KIT_PROXY not set");
   let mut config = ConductorConfig::default();
   config.network = Some(kitsune_config);
   config
}


///
pub async fn setup_1_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   /// QuicMdns Config
   // let mut network = SweetNetwork::local_quic();
   // network.network_type = kitsune_p2p::NetworkType::QuicMdns;
   // let mut config = holochain::conductor::config::ConductorConfig::default();
   // config.network = Some(network);
   // let mut conductor = SweetConductor::from_config(config).await;

   /// Standard config
   let mut conductor = SweetConductor::from_standard_config().await;

   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app", alex.clone(), &[dna.clone()])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   println!("\n\n\n SETUP DONE\n\n");

   (conductor, alex, cell1)
}


///
pub async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   // let mut network = SweetNetwork::env_var_proxy().unwrap_or_else(|| {
   //    println!("KIT_PROXY not set using local quic network");
   //    SweetNetwork::local_quic()
   // });
   // let mut network = SweetNetwork::local_quic();
   // network.network_type = kitsune_p2p::NetworkType::QuicMdns;
   // let mut config = holochain::conductor::config::ConductorConfig::default();
   // config.network = Some(network);

   // /// Common config with proxy
   //let config = create_network_config();
   //let mut conductors = SweetConductorBatch::from_config(n, config).await;

   /// Default config
   let mut conductors = SweetConductorBatch::from_standard_config(n).await;

   let all_agents: Vec<AgentPubKey> =
      future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
   println!("\n* INSTALLING APP...");
   let apps = conductors
      .setup_app_for_zipped_agents("app", &all_agents, &[dna])
      .await
      .unwrap();
   println!("\n* EXCHANGING PEER INFO...");
   conductors.exchange_peer_info().await;
   println!("\n* CONDUCTORS SETUP DONE\n\n");
   (conductors, all_agents, apps)
}


///
pub async fn setup_3_conductors() -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let (conductors, agents, apps) = setup_conductors(3).await;
   let _cells = apps.cells_flattened();

   // let _: HeaderHash = conductors[0].call(&cells[0].zome("snapmail"), "set_handle", ALEX_NICK).await;
   // let _: HeaderHash = conductors[1].call(&cells[1].zome("snapmail"), "set_handle", BILLY_NICK).await;
   // let _: HeaderHash = conductors[2].call(&cells[2].zome("snapmail"), "set_handle", CAMILLE_NICK).await;
   //
   // let _ = try_zome_call(&conductors[0], cells[0], "get_all_handles", (),
   //                                                  |handle_list: &Vec<HandleItem>| handle_list.len() == 3).await;

   println!("\n\n\n AGENTS SETUP DONE\n\n");

   (conductors, agents, apps)
}


///
fn print_element(element: &SourceChainJsonElement) -> String {
   let mut str = format!("{:?} ", element.header.header_type());
  // let mut str = format!("({}) ", element.header_address);

   // if (element.header.header_type() == HeaderType::CreateLink) {
   //    str += &format!(" '{:?}'", element.header.tag());
   // }

   match &element.header {
      Header::CreateLink(create_link) => {
         // let s = std::str::from_utf8(&create_link.tag.0).unwrap();
         let s = String::from_utf8_lossy(&create_link.tag.0).to_string();
         str += &format!("'{:.20}'", s).yellow().to_string();
      },
      Header::Create(create_entry) => {
            let mut s = String::new();
            match &create_entry.entry_type {
            EntryType::App(app_entry_type) => {
               s += "AppEntry ";
               //let entry_kind = EntryKind::from_index(&app_entry_type.id()).as_ref().to_owned();
               //s += &format!("'{}'", entry_kind);
               s += &format!("z{} e{}", u8::from(app_entry_type.zome_id()), u8::from(app_entry_type.id()));
               if app_entry_type.visibility() == &EntryVisibility::Public {
                  s = s.green().to_string();
               } else {
                  s = s.red().to_string();
               }
            },
            _ => {
               s += &format!("{:?}", create_entry.entry_type);
               s = s.green().to_string();
            }
         };
         str += &s;
      },
      Header::Update(update_entry) => {
         let mut s = String::new();
         match &update_entry.entry_type {
            EntryType::App(app_entry_type) => {
               s += "AppEntry ";
               //let entry_kind = EntryKind::from_index(&app_entry_type.id()).as_ref().to_owned();
               //s += &format!("'{}'", entry_kind).green();
               s += &format!("z{} e{}", u8::from(app_entry_type.zome_id()), u8::from(app_entry_type.id()));
            },
            _ => {
               s += &format!("{:?}", update_entry.entry_type);
            }
         };
         str += &s.yellow().to_string();
      },
      Header::DeleteLink(delete_link) => {
         let s = format!("{}", delete_link.link_add_address);
         str += &format!("'{:.25}'", s).yellow().to_string();
      },
      Header::Delete(delete_entry) => {
         let s = format!("{}", delete_entry.deletes_address);
         str += &format!("'{:.25}'", s).green().to_string();
      }
      _ => {},
   }

   //       else {
   //    if (element.header.entry_type) {
   //       if (typeof element.header.entry_type === 'object') {
   //          str += ' - AppEntry ; id = ' + element.header.entry_type.App.id
   //       } else {
   //          str += ' - ' + element.header.entry_type
   //       }
   //    }
   // }

   let mut line = format!("{:<40} ({})", str, element.header_address);

   if element.header.is_genesis() {
      line = line.blue().to_string();
   }
   line
}


///
pub async fn print_peers(conductor: &SweetConductor, cell: &SweetCell) {
   let cell_id = cell.cell_id();
   let space = cell_id.dna_hash().to_kitsune();
   let env = conductor.get_p2p_env(space);
   let peer_dump = p2p_agent_store::dump_state(
      env.into(),
      Some(cell_id.clone()),
   ).await.expect("p2p_store should not fail");
   println!(" *** peer_dump: {:?}",peer_dump.peers);
}


///
pub async fn print_chain(conductor: &SweetConductor, agent: &AgentPubKey, cell: &SweetCell) {
   let cell_id = cell.cell_id();
   let vault = conductor.get_authored_env(cell_id.dna_hash()).unwrap();

   let space = cell_id.dna_hash().to_kitsune();

   let env = conductor.get_p2p_env(space);
   let _peer_dump = p2p_agent_store::dump_state(
      env.into(),
      Some(cell_id.clone()),
   ).await.expect("p2p_store should not fail");

   // let p2p_env = conductor
   //    .p2p_env
   //    .lock()
   //    .get(&space)
   //    .cloned()
   //    .expect("invalid cell space");
   // let peer_dump = p2p_agent_store::dump_state(p2p_env.into(), Some(cell_id.clone()))?;

   //println!(" *** peer_dump: {:?}",peer_dump.peers);

   let json_dump = dump_state(vault.clone().into(), agent.clone()).await.unwrap();
   //let json = serde_json::to_string_pretty(&json_dump).unwrap();

   println!(" ====== SOURCE-CHAIN STATE DUMP START ===== {}", json_dump.elements.len());
   //println!("source_chain_dump({}) of {:?}", json_dump.elements.len(), agent);

   let mut count = 0;
   for element in &json_dump.elements {
      let str = print_element(&element);
      println!(" {:2}. {}", count, str);
      count += 1;
   }

   println!(" ====== SOURCE-CHAIN STATE DUMP END  ===== {}", json_dump.elements.len());
}


/// Call a zome function several times, waiting for a certainr result
pub async fn try_zome_call<T,P>(
   conductor: &SweetConductor,
   cell: &SweetCell,
   fn_name: &str,
   payload: P,
   predicat: fn(res: &T) -> bool,
) -> Result<T, ()>
   where
      T: serde::de::DeserializeOwned + std::fmt::Debug,
      P: Clone + serde::Serialize + std::fmt::Debug,
{
   for _ in 0..10u32 {
      let res: T = conductor.call(&cell.zome("secret"), fn_name, payload.clone())
                            .await;
      if predicat(&res) {
         return Ok(res);
      }
      tokio::time::sleep(std::time::Duration::from_millis(100)).await;
   }
   Err(())
}