pub mod dummy;
pub mod streamdeck_rs;

use std::fmt::Debug;

use async_trait::async_trait;
use tauri::async_runtime::Sender;

pub enum TransportEvent {
    Attached { serial: String },
    Detached { serial: String },
    ButtonPressed { serial: String, index: u8 },
    ButtonReleased { serial: String, index: u8 },
}

#[async_trait]
pub trait Device: Send + Sync + Debug {
    async fn open(&mut self, sender: Sender<TransportEvent>);
    async fn close(&mut self);
    async fn connected(&mut self) -> bool;
    async fn serial(&mut self) -> String;
    async fn write_image(&self, key: u8, image: String);
}

pub enum TransportType {
    // Dummy(),
    StreamdeckRs(),
}

impl Clone for TransportType {
    fn clone(&self) -> TransportType {
        match self {
            // TransportType::Dummy() => TransportType::Dummy(),
            TransportType::StreamdeckRs() => TransportType::StreamdeckRs(),
        }
    }
}

#[async_trait]
pub(crate) trait Transport: Send + Sync {
    async fn enumerate(&mut self) -> Vec<Box<dyn Device>>;
}
