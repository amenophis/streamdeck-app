use std::{collections::HashMap, sync::Arc, time::Duration};

use tauri::async_runtime::{spawn, Mutex, Sender};
use tokio::time::sleep;

use super::{manager::DeviceManager, transport::Device};
pub enum Events {
    Attached { serial: String },
    Detached { serial: String },
}

pub struct StreamDeckMonitor {
    manager: Arc<Mutex<DeviceManager>>,
    streamdecks: Arc<Mutex<HashMap<String, Box<dyn Device>>>>,
    sender: Sender<Events>,
}

impl StreamDeckMonitor {
    pub fn new(manager: Arc<Mutex<DeviceManager>>, sender: Sender<Events>) -> Self {
        Self {
            manager,
            streamdecks: Arc::new(Mutex::new(HashMap::new())),
            sender,
        }
    }
    pub async fn start(&mut self) {
        let manager = self.manager.clone();
        let streamdecks = self.streamdecks.clone();
        let sender = self.sender.clone();

        spawn(async move {
            loop {
                {
                    let mut manager = manager.lock().await;
                    let attached_streamdecks = manager.enumerate().await;

                    let mut attached_serials = Vec::new();

                    // Look for new streamdecks
                    for streamdeck in attached_streamdecks {
                        let mut streamdecks = streamdecks.lock().await;

                        let serial = streamdeck.serial().await;
                        attached_serials.push(serial.clone());

                        if !streamdecks.contains_key(&serial) {
                            // Send Attached event
                            sender
                                .send(Events::Attached {
                                    serial: serial.clone(),
                                })
                                .await;

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
                            sender
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

    pub async fn stop(&self) {}
}
