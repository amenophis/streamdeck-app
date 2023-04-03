use std::{
    fmt::{Debug, Formatter, Result},
    sync::Arc,
};

use super::{Device, Transport, TransportEvent};
use async_trait::async_trait;
use elgato_streamdeck::{
    asynchronous::ButtonStateUpdate, info::Kind, list_devices, AsyncStreamDeck,
};
use hidapi::HidApi;
use image::open;
use tauri::async_runtime::{spawn, Mutex, Sender};
use tokio::fs;

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
    async fn open(&mut self, sender: Sender<TransportEvent>) {
        let hid_api = self.hid_api.lock().await;

        let streamdeck =
            AsyncStreamDeck::connect(&hid_api, self.kind, self.serial.as_str()).unwrap();

        self.streamdeck = Some(streamdeck.clone());

        let streamdeck = streamdeck.clone();

        let serial = self.serial.clone();

        let reader = streamdeck.get_reader().clone();

        spawn(async move {
            while let Ok(button_state_updates) = reader.read(30.0).await {
                for button_state_update in button_state_updates {
                    let event = match button_state_update {
                        ButtonStateUpdate::ButtonDown(index) => TransportEvent::ButtonPressed {
                            serial: serial.clone(),
                            index,
                        },
                        ButtonStateUpdate::ButtonUp(index) => TransportEvent::ButtonReleased {
                            serial: serial.clone(),
                            index,
                        },
                    };

                    let _ = sender.send(event).await;
                }
            }
        });
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

    async fn write_image(&self, key: u8, image: String) {
        let streamdeck = self.streamdeck.as_ref().unwrap();

        let image = open("/home/jeremy/Téléchargements/pnggrad8rgb.jpg").unwrap();

        let res = streamdeck.set_button_image(key, image).await;

        match res {
            Ok(_) => {
                dbg!("OK");
                ()
            }
            Err(error) => {
                dbg!(error);
                ()
            }
        }
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
