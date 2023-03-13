use std::sync::Arc;

use tauri::{
    async_runtime::{channel, spawn, Mutex},
    AppHandle, Manager,
};

use crate::streamdeck::transport::Events;

use super::{monitor::StreamDeckMonitor, transport::TransportType};

pub struct StreamDeckServer {
    current: Option<String>,
    streamdeck_serials: Arc<Mutex<Vec<String>>>,
    transport_type: TransportType,
    monitor: Option<Arc<Mutex<StreamDeckMonitor>>>,
}

impl StreamDeckServer {
    pub fn new(transport_type: TransportType) -> Self {
        Self {
            current: None,
            streamdeck_serials: Arc::new(Mutex::new(Vec::new())),
            transport_type,
            monitor: None,
        }
    }

    pub async fn start(&mut self, app_handle: AppHandle) {
        self.start_app(app_handle.clone()).await;
        self.start_monitor().await;
    }

    async fn start_monitor(&mut self) {
        if self.monitor.is_none() {
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

            let monitor = Arc::new(Mutex::new(StreamDeckMonitor::new(
                self.transport_type.clone(),
                events_sender.clone(),
            )));

            self.monitor = Some(monitor);
        }

        let monitor = self.monitor.as_ref().unwrap();
        let mut monitor = monitor.lock().await;

        monitor.start().await;
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
