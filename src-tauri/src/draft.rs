// #[derive(Clone, serde::Serialize)]
// struct StreamdeckAttachedPayload {
//     serial: String,
//     name: String,
//     row_count: u8,
//     column_count: u8,
//     key_count: u8,
//     kind: String,
// }

// let hid_api = HidApi::new();
// if hid_api.is_err() {
//     panic!("Unable to open hid_api");
// }

// let global_hid_api = Arc::new(Mutex::new(hid_api.unwrap()));
// let hid_api = global_hid_api.clone();

// let streamdecks = Arc::new(Mutex::new(StreamdeckMap::new()));

// let global_app_handle = app.handle();
// let app_handle = app.handle();

// let (key_event_tx, mut key_event_rx): (Sender<KeyEvent>, Receiver<KeyEvent>) = channel(32);
// let (command_tx, mut command_rx): (Sender<StreamdeckCommand>, Receiver<StreamdeckCommand>) =
//     channel(32);

// let key_event_tx2 = key_event_tx.clone();
// let command_tx2 = command_tx.clone();
// let command_tx3 = command_tx.clone();

// spawn(async move {
//     while let Some(message) = key_event_rx.recv().await {
//         let command = match message {
//             KeyEvent::Up { serial, key: _ } => StreamdeckCommand::Firmware {
//                 serial: serial.clone(),
//             },
//             KeyEvent::Down { serial, key: _ } => StreamdeckCommand::Firmware {
//                 serial: serial.clone(),
//             },
//         };

//         match command_tx3.send(command).await {
//             Ok(()) => {
//                 println!("KeyEvent sent");
//             }
//             Err(error) => {
//                 println!("Problem sending KeyEvent {:?}", error.to_string());
//             }
//         };
//     }
// });

// let streamdecks_command = streamdecks.clone();

// spawn(async move {
//     while let Some(command) = command_rx.recv().await {
//         println!("Command loop");
//         match command {
//             StreamdeckCommand::Firmware { serial } => {
//                 println!(
//                     "{}",
//                     streamdecks_command
//                         .lock()
//                         .await
//                         .get(&serial)
//                         .unwrap()
//                         .0
//                         .firmware_version()
//                         .await
//                         .unwrap()
//                 );
//             }
//         }
//     }
// });

// spawn(async move {
//     println!("Starting loop");
//     loop {
//         let mut hid_api = hid_api.lock().await;

//         let mut streamdecks = streamdecks.lock().await;

//         let _ = hid_api.refresh_devices();

//         let attached_streamdecks = list_devices(&hid_api);
//         let mut attached_serials = Vec::new();

//         // Add new streamdeck
//         for (kind, serial) in attached_streamdecks {
//             attached_serials.push(serial.clone());

//             if streamdecks.contains_key(&serial) {
//                 continue;
//             }

//             println!("Connect ");
//             if let Ok(device) = AsyncStreamDeck::connect(&hid_api, kind, &serial) {
//                 println!("Connected ");

//                 streamdecks.insert(serial.clone(), (device.clone(), command_tx2.clone()));
//                 let _ = app_handle.emit_all(
//                     "device_attached",
//                     StreamdeckAttachedPayload {
//                         serial: serial.clone(),
//                         name: match kind {
//                             Kind::Original => "Stream Deck".to_string(),
//                             Kind::OriginalV2 => "Stream Deck V2".to_string(),
//                             Kind::Mini => "Stream Deck Mini".to_string(),
//                             Kind::Xl => "Stream Deck XL".to_string(),
//                             Kind::XlV2 => "Stream Deck XL V2".to_string(),
//                             Kind::Mk2 => "Stream Deck Mk2".to_string(),
//                             Kind::MiniMk2 => "Stream Mini Deck Mk2".to_string(),
//                             Kind::Pedal => "Stream Deck Pedal".to_string(),
//                         },
//                         row_count: kind.row_count(),
//                         column_count: kind.column_count(),
//                         key_count: kind.key_count(),
//                         kind: match kind {
//                             Kind::Original => "original".to_string(),
//                             Kind::OriginalV2 => "original_v2".to_string(),
//                             Kind::Mini => "mini".to_string(),
//                             Kind::Xl => "xl".to_string(),
//                             Kind::XlV2 => "xl_v2".to_string(),
//                             Kind::Mk2 => "mk2".to_string(),
//                             Kind::MiniMk2 => "mini_mk2".to_string(),
//                             Kind::Pedal => "pedal".to_string(),
//                         },
//                     },
//                 );

//                 println!("Attached {}", serial);

//                 start_listen_keys(device.clone(), key_event_tx2.clone());
//             }
//         }

//         // Remove not connected streamdeck
//         let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
//         for serial in streamdecks.keys() {
//             if !attached_serials.contains(&serial) {
//                 to_remove.push(serial.clone());
//             }
//         }
//         for serial in to_remove {
//             streamdecks.remove(&serial);
//             let _ = app_handle.emit_all("device_detached", serial.clone());
//             println!("Detached {}", serial);
//         }
//     }
// });

// fn start_listen_keys(device: Arc<AsyncStreamDeck>, channel: Sender<KeyEvent>) {
//     spawn(async move {
//         let serial = device.serial_number().await.unwrap();
//         let reader = device.get_reader();

//         loop {
//             match reader.read(100.0).await {
//                 Ok(states) => {
//                     for s in states {
//                         let message = match s {
//                             ButtonStateUpdate::ButtonDown(key) => KeyEvent::Down {
//                                 serial: serial.clone(),
//                                 key,
//                             },
//                             ButtonStateUpdate::ButtonUp(key) => KeyEvent::Up {
//                                 serial: serial.clone(),
//                                 key,
//                             },
//                         };

//                         match channel.send(message).await {
//                             Ok(()) => {
//                                 println!("KeyEvent sent");
//                             }
//                             Err(error) => {
//                                 println!("Problem sending KeyEvent {:?}", error.to_string());
//                             }
//                         };
//                     }
//                 }
//                 Err(error) => {
//                     println!(
//                         "Problem reading button states on {}: {:?}",
//                         serial,
//                         error.to_string()
//                     );

//                     break;
//                 }
//             };
//         }
//     });
// }

//         // let mut streamdecks = streamdecks.lock().await;

//         // let _ = hid_api.refresh_devices();

//         // let attached_streamdecks = list_devices(&hid_api);
//         // let mut attached_serials = Vec::new();

//         // // Add new streamdeck
//         // for (kind, serial) in attached_streamdecks {
//         //     attached_serials.push(serial.clone());

//         //     if streamdecks.contains_key(&serial) {
//         //         continue;
//         //     }

//         //     println!("Connect ");
//         //     if let Ok(device) = AsyncStreamDeck::connect(&hid_api, kind, &serial) {
//         //         println!("Connected ");

//         //         streamdecks.insert(serial.clone(), (device.clone(), command_tx2.clone()));
//         //         let _ = app_handle.emit_all(
//         //             "device_attached",
//         //             StreamdeckAttachedPayload {
//         //                 serial: serial.clone(),
//         //                 name: match kind {
//         //                     Kind::Original => "Stream Deck".to_string(),
//         //                     Kind::OriginalV2 => "Stream Deck V2".to_string(),
//         //                     Kind::Mini => "Stream Deck Mini".to_string(),
//         //                     Kind::Xl => "Stream Deck XL".to_string(),
//         //                     Kind::XlV2 => "Stream Deck XL V2".to_string(),
//         //                     Kind::Mk2 => "Stream Deck Mk2".to_string(),
//         //                     Kind::MiniMk2 => "Stream Mini Deck Mk2".to_string(),
//         //                     Kind::Pedal => "Stream Deck Pedal".to_string(),
//         //                 },
//         //                 row_count: kind.row_count(),
//         //                 column_count: kind.column_count(),
//         //                 key_count: kind.key_count(),
//         //                 kind: match kind {
//         //                     Kind::Original => "original".to_string(),
//         //                     Kind::OriginalV2 => "original_v2".to_string(),
//         //                     Kind::Mini => "mini".to_string(),
//         //                     Kind::Xl => "xl".to_string(),
//         //                     Kind::XlV2 => "xl_v2".to_string(),
//         //                     Kind::Mk2 => "mk2".to_string(),
//         //                     Kind::MiniMk2 => "mini_mk2".to_string(),
//         //                     Kind::Pedal => "pedal".to_string(),
//         //                 },
//         //             },
//         //         );

//         //         println!("Attached {}", serial);

//         //         start_listen_keys(device.clone(), key_event_tx2.clone());
//         //     }
//         // }

//         // // Remove not connected streamdeck
//         // let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
//         // for serial in streamdecks.keys() {
//         //     if !attached_serials.contains(&serial) {
//         //         to_remove.push(serial.clone());
//         //     }
//         // }
//         // for serial in to_remove {
//         //     streamdecks.remove(&serial);
//         //     let _ = app_handle.emit_all("device_detached", serial.clone());
//         //     println!("Detached {}", serial);
//         // }

//         sleep(Duration::from_secs(1));
//     }
// })
// .await;
