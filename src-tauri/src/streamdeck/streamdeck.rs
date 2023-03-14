use std::{collections::HashMap, sync::Arc, time::Duration};

use tauri::async_runtime::{spawn, Mutex, Sender};
use tokio::time::sleep;

use super::transport::{streamdeck_rs::StreamdeckRs, Device, Events, Transport, TransportType};

pub struct StreamDeckManager {
    transport: Arc<Mutex<Box<dyn Transport>>>,
    streamdecks: Arc<Mutex<HashMap<String, Box<dyn Device>>>>,
    sender: Sender<Events>,
}

impl StreamDeckManager {
    pub fn new(t: TransportType, sender: Sender<Events>) -> Self {
        Self {
            transport: match t {
                // TransportType::Dummy() => Box::new(Dummy::new()),
                TransportType::StreamdeckRs() => {
                    Arc::new(Mutex::new(Box::new(StreamdeckRs::new())))
                }
            },
            streamdecks: Arc::new(Mutex::new(HashMap::new())),
            sender,
        }
    }
    pub async fn start(&mut self) {
        let streamdecks = self.streamdecks.clone();
        let sender = self.sender.clone();

        let transport = self.transport.clone();

        spawn(async move {
            loop {
                {
                    let mut transport = transport.lock().await;

                    let attached_streamdecks = transport.enumerate().await;

                    let mut attached_serials = Vec::new();

                    // Look for new streamdecks
                    for mut streamdeck in attached_streamdecks {
                        let mut streamdecks = streamdecks.lock().await;

                        let serial = streamdeck.serial().await;
                        attached_serials.push(serial.clone());

                        if !streamdecks.contains_key(&serial) {
                            // Send Attached event
                            let _ = sender
                                .send(Events::Attached {
                                    serial: serial.clone(),
                                })
                                .await;

                            streamdeck.open(sender.clone()).await;

                            // Attach streamdeck
                            streamdecks.insert(serial, streamdeck);
                        }
                    }

                    // Look for suspended streamdecks
                    // TODO later

                    {
                        let mut streamdecks = streamdecks.lock().await;

                        // Remove unplugged streamdecks
                        let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
                        for serial in streamdecks.keys() {
                            if !attached_serials.contains(&serial) {
                                to_remove.push(serial.clone());
                            }
                        }
                        for serial in to_remove {
                            streamdecks.remove(&serial);

                            // Send Detached event
                            let _ = sender
                                .send(Events::Detached {
                                    serial: serial.clone(),
                                })
                                .await;
                        }
                    }
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // pub async fn stop(&self) {}
}
