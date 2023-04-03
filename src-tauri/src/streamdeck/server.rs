use std::{collections::HashMap, sync::Arc, time::Duration};

use tauri::{
    async_runtime::{channel, spawn, Mutex, Receiver, Sender},
    AppHandle, Manager,
};
use tokio::{fs, time::sleep};

use crate::streamdeck::transport::TransportEvent;

use super::{
    plugin::{plugin_events::PluginEvent, Plugin},
    transport::{streamdeck_rs::StreamdeckRs, Device, Transport, TransportType},
};

const PLUGINS_PATH: &'static str = "/home/jeremy/.config/streamdeck-app/plugins/"; // TODO: make this dynamic

pub struct StreamDeckServer {
    transport: Arc<Mutex<Box<dyn Transport>>>,
    streamdecks: Arc<Mutex<HashMap<String, Box<dyn Device>>>>,
    plugins: HashMap<String, Plugin>,
    current: Option<String>,
    streamdeck_serials: Arc<Mutex<Vec<String>>>,
}

impl StreamDeckServer {
    pub async fn new(transport_type: TransportType, app_handle: AppHandle) -> Self {
        let (transport_event_sender, transport_event_receiver) = channel(32);
        let (plugin_event_sender, plugin_event_receiver) = channel(32);

        let mut server = Self {
            plugins: HashMap::new(),
            current: None,
            streamdeck_serials: Arc::new(Mutex::new(Vec::new())),
            transport: match transport_type {
                // TransportType::Dummy() => Box::new(Dummy::new()),
                TransportType::StreamdeckRs() => {
                    Arc::new(Mutex::new(Box::new(StreamdeckRs::new())))
                }
            },
            streamdecks: Arc::new(Mutex::new(HashMap::new())),
        };

        server
            .start_app(app_handle.clone())
            .await
            .start_transport_event_receiver(transport_event_receiver)
            .await
            .start_plugin_event_receiver(plugin_event_receiver)
            .await
            .start_watcher(transport_event_sender)
            .await
            .start_plugins(app_handle.clone(), plugin_event_sender)
            .await;

        server
    }

    pub async fn start_plugins(
        &mut self,
        app_handle: AppHandle,
        plugin_event_sender: Sender<PluginEvent>,
    ) -> &mut Self {
        let mut reader = fs::read_dir(PLUGINS_PATH).await.unwrap();

        loop {
            if let Some(f) = reader.next_entry().await.unwrap() {
                let plugin_name = f.file_name().to_str().unwrap().to_string();
                if !plugin_name.ends_with(".sdPlugin") {
                    continue;
                }
                let path = f.path().to_str().unwrap().to_string();

                let mut plugin = Plugin::new(path).await;
                plugin
                    .start(app_handle.clone(), plugin_event_sender.clone())
                    .await;

                self.plugins.insert(plugin_name, plugin);
            } else {
                break;
            }
        }

        self
    }
    async fn start_transport_event_receiver(
        &mut self,
        mut events_receiver: Receiver<TransportEvent>,
    ) -> &mut Self {
        let streamdeck_serials = self.streamdeck_serials.clone();
        let mut current = self.current.clone();

        spawn(async move {
            while let Some(event) = events_receiver.recv().await {
                match event {
                    TransportEvent::Attached { serial } => {
                        if current.is_none() {
                            current = Some(serial.clone());
                        }
                        streamdeck_serials.lock().await.push(serial.clone());
                        println!("Attached {:#?}", serial);
                    }
                    TransportEvent::Detached { serial } => {
                        streamdeck_serials.lock().await.retain(|s| s != &serial);
                        println!("Detached {}", serial);
                    }
                    TransportEvent::ButtonPressed { serial, index } => {
                        println!("Button Pressed {}: {}", serial, index);
                    }
                    TransportEvent::ButtonReleased { serial, index } => {
                        println!("Button Released {}: {}", serial, index);
                    }
                };
                sleep(Duration::from_secs(1)).await;
            }
        });

        self
    }

    async fn start_plugin_event_receiver(
        &mut self,
        mut plugin_event_receiver: Receiver<PluginEvent>,
    ) -> &mut Self {
        let streamdecks = self.streamdecks.clone();
        spawn(async move {
            while let Some(event) = plugin_event_receiver.recv().await {
                match event {
                    PluginEvent::SetImage { context, payload } => {
                        let streamdecks = streamdecks.lock().await;
                        streamdecks
                            .get("AL32K2C65582")
                            .unwrap()
                            .write_image(0, payload.image)
                            .await;
                    }
                    _ => {}
                };

                sleep(Duration::from_secs(1)).await;
            }
        });

        self
    }

    async fn start_watcher(&mut self, transport_event_sender: Sender<TransportEvent>) -> &mut Self {
        let streamdecks = self.streamdecks.clone();

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
                            let _ = transport_event_sender
                                .send(TransportEvent::Attached {
                                    serial: serial.clone(),
                                })
                                .await;

                            streamdeck.open(transport_event_sender.clone()).await;

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
                            let _ = transport_event_sender
                                .send(TransportEvent::Detached {
                                    serial: serial.clone(),
                                })
                                .await;
                        }
                    }
                }

                sleep(Duration::from_secs(10)).await;
            }
        });

        self
    }

    async fn start_app(&mut self, app_handle: AppHandle) -> &mut Self {
        let id = app_handle.listen_global("to-be-defined", |event| {
            println!("got event-name with payload {:?}", event.payload());
        });

        app_handle.unlisten(id);

        self
    }
}
