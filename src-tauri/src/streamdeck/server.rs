use std::{collections::HashMap, sync::Arc};

use tauri::async_runtime::{channel, spawn, Mutex};

use super::{
    manager::DeviceManager,
    monitor::{Events, StreamDeckMonitor},
    transport::Device,
};

pub struct StreamDeckServer {
    decks: HashMap<String, Box<dyn Device>>,
    deck_ids: HashMap<String, Box<dyn Device>>,
    monitor: Option<Arc<Mutex<StreamDeckMonitor>>>,
}

impl StreamDeckServer {
    pub fn new() -> Self {
        Self {
            decks: HashMap::new(),
            deck_ids: HashMap::new(),
            monitor: None,
        }
    }

    pub async fn start(&mut self, device_manager: DeviceManager) {
        if self.monitor.is_none() {
            let (events_sender, mut events_receiver) = channel(32);

            spawn(async move {
                while let Some(event) = events_receiver.recv().await {
                    match event {
                        Events::Attached { serial } => {
                            println!("Attached {}", serial);
                        }
                        Events::Detached { serial } => {
                            println!("Detached {}", serial);
                        }
                    };
                }
            });

            let manager = Arc::new(Mutex::new(device_manager));
            let monitor = Arc::new(Mutex::new(StreamDeckMonitor::new(
                manager,
                events_sender.clone(),
            )));

            self.monitor = Some(monitor);
        }

        if self.monitor.is_some() {
            let monitor = self.monitor.as_ref().unwrap();
            let mut monitor = monitor.lock().await;

            monitor.start().await;
        }
    }

    pub async fn stop(&mut self) {
        if self.monitor.is_some() {
            let monitor = self.monitor.as_ref().unwrap();
            let monitor = monitor.lock().await;

            monitor.stop().await;
        }
    }

    pub fn get_deck(&mut self, serial: String) {
        todo!()
    }
}
