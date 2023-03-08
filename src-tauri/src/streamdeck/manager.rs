use super::transport::{streamdeck_rs::StreamdeckRs, Device, Transport, TransportType};

pub struct DeviceManager {
    transport: Box<dyn Transport>,
}

impl DeviceManager {
    fn get_transport(t: TransportType) -> Box<dyn Transport> {
        match t {
            // TransportType::Dummy() => Box::new(Dummy::new()),
            TransportType::StreamdeckRs() => Box::new(StreamdeckRs::new()),
        }
    }

    pub async fn enumerate(&mut self) -> Vec<Box<dyn Device>> {
        self.transport.enumerate().await
    }

    pub fn new(t: TransportType) -> Self {
        Self {
            transport: DeviceManager::get_transport(t),
        }
    }
}
