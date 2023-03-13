use std::{
    fmt::{Debug, Formatter, Result},
    sync::Arc,
};

use super::{Device, Events, Transport};
use async_trait::async_trait;
use elgato_streamdeck::{
    asynchronous::ButtonStateUpdate, info::Kind, list_devices, AsyncStreamDeck,
};
use hidapi::HidApi;
use tauri::async_runtime::{spawn, Mutex, Sender};

struct StreamdeckRsDevice {
    hid_api: Arc<Mutex<HidApi>>,
    kind: Kind,
    serial: String,
    streamdeck: Option<Arc<AsyncStreamDeck>>,
}

impl StreamdeckRsDevice {
    fn new(hid_api: Arc<Mutex<HidApi>>, kind: Kind, serial: String) -> Self {
        Self {
            hid_api,
            kind,
            serial,
            streamdeck: None,
        }
    }
}

impl Debug for StreamdeckRsDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("")
            .field(&self.kind)
            .field(&self.serial)
            .finish()
    }
}

#[async_trait]
impl Device for StreamdeckRsDevice {
    async fn open(&mut self, sender: Sender<Events>) {
        let hid_api = self.hid_api.lock().await;

        let streamdeck =
            AsyncStreamDeck::connect(&hid_api, self.kind, self.serial.as_str()).unwrap();

        self.streamdeck = Some(streamdeck.clone());

        let streamdeck = streamdeck.clone();

        let serial = self.serial.clone();

        let reader = streamdeck.get_reader().clone();

        let _ = spawn(async move {
            while let Ok(button_state_updates) = reader.read(30.0).await {
                for button_state_update in button_state_updates {
                    let event = match button_state_update {
                        ButtonStateUpdate::ButtonDown(index) => Events::ButtonPressed {
                            serial: serial.clone(),
                            index,
                        },
                        ButtonStateUpdate::ButtonUp(index) => Events::ButtonReleased {
                            serial: serial.clone(),
                            index,
                        },
                    };

                    let _ = sender.send(event).await;
                }
            }
        })
        .await;
    }

    async fn close(&mut self) {
        todo!()
    }

    async fn connected(&mut self) -> bool {
        match self.streamdeck.clone() {
            Some(_d) => true,
            None => false,
        }
    }

    async fn serial(&mut self) -> String {
        self.serial.clone()
    }
}

pub struct StreamdeckRs {
    hid_api: Arc<Mutex<HidApi>>,
}

impl StreamdeckRs {
    pub fn new() -> Self {
        let hid_api = HidApi::new();
        if hid_api.is_err() {
            panic!("Unable to open hid_api");
        }

        Self {
            hid_api: Arc::new(Mutex::new(hid_api.unwrap())),
        }
    }
}

#[async_trait]
impl Transport for StreamdeckRs {
    async fn enumerate(&mut self) -> Vec<Box<dyn Device>> {
        let mut devices: Vec<Box<dyn Device>> = Vec::new();

        {
            let mut hid_api = self.hid_api.lock().await;

            let _ = hid_api.refresh_devices();

            for (kind, serial) in list_devices(&hid_api) {
                let hid_api_arc = self.hid_api.clone();
                let streamdeck = StreamdeckRsDevice::new(hid_api_arc, kind, serial.clone());

                let b = Box::new(streamdeck);

                let device = b as Box<dyn Device>;

                devices.push(device);
            }
        }

        devices
    }
}
