use std::sync::Arc;

use tauri::{
    async_runtime::{channel, spawn, Mutex},
    AppHandle, Manager,
};

use crate::streamdeck::transport::Events;

use super::{plugin::PluginManager, streamdeck::StreamDeckManager, transport::TransportType};

pub struct StreamDeckServer {
    current: Option<String>,
    streamdeck_serials: Arc<Mutex<Vec<String>>>,
    transport_type: TransportType,
    plugin_manager: Option<Arc<Mutex<PluginManager>>>,
    streamdeck_manager: Option<Arc<Mutex<StreamDeckManager>>>,
}

impl StreamDeckServer {
    pub fn new(transport_type: TransportType) -> Self {
        Self {
            current: None,
            streamdeck_serials: Arc::new(Mutex::new(Vec::new())),
            transport_type,
            plugin_manager: None,
            streamdeck_manager: None,
        }
    }

    pub async fn start(&mut self, app_handle: AppHandle) {
        self.start_app(app_handle.clone()).await;
        self.start_plugin_manager(app_handle.clone()).await;
        self.start_streamdeck_manager().await;
    }

    async fn start_plugin_manager(&mut self, app_handle: AppHandle) {
        if self.plugin_manager.is_none() {
            self.plugin_manager =
                Some(Arc::new(Mutex::new(PluginManager::new(app_handle.clone()))));
        }

        let plugin_manager = self.plugin_manager.as_ref().unwrap();
        let mut plugin_manager = plugin_manager.lock().await;

        plugin_manager.start().await;
    }

    async fn start_streamdeck_manager(&mut self) {
        if self.streamdeck_manager.is_none() {
            let (events_sender, mut events_receiver) = channel(32);

            let streamdeck_serials = self.streamdeck_serials.clone();
            let mut current = self.current.clone();

            spawn(async move {
                while let Some(event) = events_receiver.recv().await {
                    match event {
                        Events::Attached { serial } => {
                            if current.is_none() {
                                current = Some(serial.clone());
                            }
                            streamdeck_serials.lock().await.push(serial.clone());
                            println!("Attached {:#?}", serial);
                        }
                        Events::Detached { serial } => {
                            streamdeck_serials.lock().await.retain(|s| s != &serial);
                            println!("Detached {}", serial);
                        }
                        Events::ButtonPressed { serial, index } => {
                            println!("Button Pressed {}: {}", serial, index);
                        }
                        Events::ButtonReleased { serial, index } => {
                            println!("Button Released {}: {}", serial, index);
                        }
                    };
                }
            });

            self.streamdeck_manager = Some(Arc::new(Mutex::new(StreamDeckManager::new(
                self.transport_type.clone(),
                events_sender.clone(),
            ))));
        }

        let streamdeck_manager = self.streamdeck_manager.as_ref().unwrap();
        let mut streamdeck_manager = streamdeck_manager.lock().await;

        streamdeck_manager.start().await;
    }

    async fn start_app(&mut self, app_handle: AppHandle) {
        let id = app_handle.listen_global("to-be-defined", |event| {
            println!("got event-name with payload {:?}", event.payload());
        });

        app_handle.unlisten(id);
    }

    // pub async fn stop(&mut self) {
    //     if self.monitor.is_some() {
    //         let monitor = self.monitor.as_ref().unwrap();
    //         let monitor = monitor.lock().await;

    //         monitor.stop().await;
    //     }
    // }
}
