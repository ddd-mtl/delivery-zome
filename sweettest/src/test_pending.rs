
//
// /// WARNING: shutdown doesn't work
// pub async fn test_delivery_pending() {
//    /// Setup
//    let (mut conductors, agents, apps) = setup_3_conductors().await;
//    let cells = apps.cells_flattened();
//
//    // /// Setup
//    // let (mut conductor0, alex, cell0) = setup_1_conductor().await;
//    // /// Setup Billy
//    // let billy;
//    // {
//    //    let (mut conductor1, billy_temp, cell1) = setup_1_conductor().await;
//    //    let _: ActionHash = conductor1.call(&cell1.zome("snapmail"), "set_handle", BILLY_NICK).await;
//    //    billy = billy_temp.clone();
//    //    conductor1.shutdown().await;
//    // }
//    // /// Setup Camille
//    // let (mut conductor2, camille, cell2) = setup_1_conductor().await;
//    // //let mut conductors = vec![&mut conductor1, &mut conductor2, &mut conductor3];
//    // let _agents = vec![&alex, &billy, &camille];
//    // //let cells = vec![&cell0, &cell1, &cell2];
//    //
//    // let _: ActionHash = conductor0.call(&cell0.zome("snapmail"), "set_handle", ALEX_NICK).await;
//    //
//    // let _: ActionHash = conductor2.call(&cell2.zome("snapmail"), "set_handle", CAMILLE_NICK).await;
//
//    // consistency_10s(cells.as_slice()).await;
//    //println!("consistency done!");
//
//
//    /// B goes offline
//    conductors[1].shutdown().await;
//
//    // let enc_key: holochain_zome_types::X25519PubKey = conductors[1].call(&cells[1].zome("snapmail"), "get_my_enc_key", ()).await;
//
//    //consistency_10s(&cells).await;
//
//    //println!("agents: {:?}", agents);
//
//    //println!("\n\n\n SETUP DONE\n\n");
//
//
//    /// A sends to B
//    let mail = SendMailInput {
//       subject: "test-outmail".to_string(),
//       payload: "blablabla".to_string(),
//       to: vec![agents[1].clone()], // agents,
//       cc: vec![],
//       bcc: vec![],
//       manifest_address_list: vec![],
//    };
//    let outmail_hh: ActionHash = conductors[0].call(
//       &cells[0].zome("snapmail"),
//       "send_mail",
//       mail,
//    ).await;
//    println!("outmail_hh: {:?}", outmail_hh);
//
//    sleep(Duration::from_millis(20 * 1000)).await;
//
//    /// Check status: Should be 'Pending'
//    /// B checks inbox
//    try_zome_call(&conductors[0], cells[0], "get_outmail_state", outmail_hh.clone(),
//                  |mail_state: &OutMailState| {mail_state == &OutMailState::AllSent })
//       .await
//       .expect("Should have AllSent state");
//
//
//    print_chain(&conductors[0], &agents[0], &cells[0]).await;
//
//    /// B goes online
//    conductors[1].startup().await;
//
//    print_chain(&conductors[1], &agents[1], &cells[1]).await;
//    sleep(Duration::from_millis(30 * 1000)).await;
//    print_chain(&conductors[1], &agents[1], &cells[1]).await;
//
//    /// B checks inbox
//    try_zome_call(&conductors[1], cells[1], "check_mail_inbox", (), |res:&Vec<ActionHash>| {res.len() > 0})
//       .await
//       .expect("Should have one mail");
//    let mail_hhs = try_zome_call(&conductors[1], cells[1], "get_all_unacknowledged_inmails", (), |res:&Vec<ActionHash>| {res.len() > 0})
//       .await
//       .expect("Should have one mail");
//
//    /// B acknowledges mail
//    let outack_eh: EntryHash = conductors[1].call(
//       &cells[1].zome("snapmail"),
//       "acknowledge_mail",
//       mail_hhs[0].clone(),
//    ).await;
//    println!("outack_eh: {:?}", outack_eh);
//
//
//    /// A checks ack inbox
//    let outmails_ehs = try_zome_call(&conductors[0], cells[0], "check_ack_inbox", (), |res:&Vec<EntryHash>| {res.len() > 0})
//       .await
//       .expect("Should have one ack");
//    println!("outmails_ehs: {:?}", outmails_ehs);
//    try_zome_call(&conductors[0], cells[0], "get_outmail_state", outmail_hh.clone(),
//                  |mail_state: &OutMailState| {mail_state == &OutMailState::AllAcknowledged })
//       .await
//       .expect("Should have FullyAcknowledged state");
//
//    print_chain(&conductors[0], &agents[0], &cells[0]).await;
// }
