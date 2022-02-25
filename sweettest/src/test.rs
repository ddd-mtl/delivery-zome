use std::time::SystemTime;
use holochain::sweettest::*;
use holochain::conductor::{
   ConductorHandle,
};
use maplit::hashset;
//use holo_hash::*;

use crate::setup::*;
use crate::test_delivery::*;
use crate::print::*;

///
pub async fn test(arg: String) {
   let now = SystemTime::now();

   // Admin API test
   if arg == "" {
      test_list_apps().await;
   }
   // Pub Key
   if arg == "all" || arg == "key" {
      test_pub_enc_key().await;
   }
   // // Deliver to self
   // if arg == "all" || arg == "self" {
   //    test_delivery_self().await;
   // }
   // Deliver via DM
   if arg == "all" || arg == "dm" {
      test_delivery_dm().await;
   }
   // // Deliver via DHT
   // if arg == "all" || arg == "pending" {
   //    test_delivery_pending().await;
   // }
   // Deliver via DM
   if arg == "all" || arg == "dm_manifest" {
      test_delivery_dm_manifest().await;
   }
   // // Deliver via DHT
   // if arg == "all" || arg == "pending_chunks" {
   //    test_delivery_pending_chunks().await;
   // }

   // Print elapsed
   match now.elapsed() {
      Ok(elapsed) => {
         // it prints '2'
         println!("\n *** Test(s) duration: {} secs", elapsed.as_secs());
      }
      Err(e) => {
         // an error occurred!
         println!("Error: {:?}", e);
      }
   }
}


///
pub async fn test_list_apps() {
   //observability::test_run().ok();

   println!("Loading DNA...");
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   println!("INSTALLING TWO APPS...");
   // Install two apps on the Conductor:
   // Both share a CellId in common, and also include a distinct CellId each.
   let mut conductor = SweetConductor::from_standard_config().await;
   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app1", alex.clone(), &[dna.clone()])
      .await
      .unwrap();
   let _app2 = conductor
      .setup_app_for_agent("app2", alex.clone(), &[dna])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   println!("\n LIST RUNNING APPS...");
   let list_apps = |conductor: ConductorHandle, cell: SweetCell| async move {
      conductor
         .list_running_apps_for_required_cell_id(cell.cell_id())
         .await
         .unwrap()
   };
   let res = list_apps(conductor.clone(), cell1.clone()).await;
   println!("list_apps = {:?}", res);

   // - Ensure that the first CellId is associated with both apps,
   //   and the other two are only associated with one app each.
   assert_eq!(res, hashset!["app1".to_string(), "app2".to_string()]);
}


///
pub async fn test_pub_enc_key() {
   let (conductor, alex, cell1, entry_names) = setup_1_conductor().await;

   println!("Calling get_my_enc_key()");
   let enc_key: holochain_zome_types::X25519PubKey = conductor.call(&cell1.zome("delivery"), "get_my_enc_key", ()).await;
   println!("enc_key: {:?}", enc_key);
   //assert_eq!("<noname>", handle);

   print_chain(&conductor, &alex, &cell1, entry_names).await;

   //let _ :() = conductor.call(&cell1.zome("snapmail"), "init_caps", ()).await;

   //let _enc_key: holochain_zome_types::X25519PubKey = conductor.call(&cell1.zome("snapmail"), "get_my_enc_key", ()).await;

   //let _handle_address1: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", "toto").await;
}
