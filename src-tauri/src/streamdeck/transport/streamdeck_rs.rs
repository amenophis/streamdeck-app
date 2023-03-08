use std::{collections::HashMap, sync::Arc};

use super::{Device, Transport};
use async_trait::async_trait;
use elgato_streamdeck::{info::Kind, list_devices, AsyncStreamDeck};
use hidapi::HidApi;
use tauri::async_runtime::Mutex;

struct StreamdeckRsDevice {
    device: Arc<AsyncStreamDeck>,
}

impl StreamdeckRsDevice {
    fn new(hid_api: &HidApi, kind: Kind, serial: String) -> Self {
        Self {
            device: AsyncStreamDeck::connect(hid_api, kind, &serial).unwrap(),
        }
    }
}

#[async_trait]
impl Device for StreamdeckRsDevice {
    fn close(&self) {
        todo!()
    }

    fn is_open(&self) {
        todo!()
    }

    fn connected(&self) {
        todo!()
    }

    async fn serial(&self) -> String {
        self.device.serial_number().await.unwrap()
    }

    fn vendor_id(&self) {
        todo!()
    }

    fn product_id(&self) {
        todo!()
    }

    fn write_feature(&self, payload: String) {
        todo!()
    }

    fn read_feature(&self, report_id: String, length: String) {
        todo!()
    }

    fn write(&self, payload: String) {
        todo!()
    }

    fn read(&self, length: u8) {
        todo!()
    }
}

pub struct StreamdeckRs {
    hid_api: Arc<Mutex<HidApi>>,
    streamdecks: HashMap<String, Arc<AsyncStreamDeck>>,
    kinds: HashMap<String, Kind>,
}

impl StreamdeckRs {
    pub fn new() -> Self {
        let hid_api = HidApi::new();
        if hid_api.is_err() {
            panic!("Unable to open hid_api");
        }

        Self {
            hid_api: Arc::new(Mutex::new(hid_api.unwrap())),
            streamdecks: HashMap::new(),
            kinds: HashMap::new(),
        }
    }
}

#[async_trait]
impl Transport for StreamdeckRs {
    async fn enumerate(&mut self) -> Vec<Box<dyn Device>> {
        let mut devices: Vec<Box<dyn Device>> = Vec::new();
        self.kinds.clear();

        {
            let mut hid_api = self.hid_api.lock().await;

            let _ = hid_api.refresh_devices();

            for (kind, serial) in list_devices(&hid_api) {
                let streamdeck = StreamdeckRsDevice::new(&hid_api, kind, serial.clone());

                let b = Box::new(streamdeck);

                let device = b as Box<dyn Device>;

                devices.push(device);
                self.kinds.insert(serial.clone(), kind.clone());
            }
        }

        devices
    }
}
