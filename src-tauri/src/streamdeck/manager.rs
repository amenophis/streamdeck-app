use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;
use hidapi::HidApi;
use tauri::{AppHandle, Manager as TauriManager, Wry};

use crate::streamdeck::devices::streamdeck_mini::StreamdeckMini;
use crate::streamdeck::devices::streamdeck_mk2::StreamdeckMk2;
use crate::streamdeck::devices::streamdeck_original::StreamdeckOriginal;
use crate::streamdeck::devices::streamdeck_original_v2::StreamdeckOriginalV2;
use crate::streamdeck::devices::streamdeck_xl::StreamdeckXl;

const VENDOR_ID: u16 = 0x0fd9;

const PRODUCT_ID_STREAMDECK_MINI: u16 = 0x0063;
const PRODUCT_ID_STREAMDECK_MK2: u16 = 0x0080;
const PRODUCT_ID_STREAMDECK_ORIGINAL: u16 = 0x0060;
const PRODUCT_ID_STREAMDECK_ORIGINAL_V2: u16 = 0x006d;
const PRODUCT_ID_STREAMDECK_XL: u16 = 0x006c;

type StreamdeckProduct = (u16, u16);

static STREAMDECK_PRODUCTS: [StreamdeckProduct; 5] = [
    (VENDOR_ID, PRODUCT_ID_STREAMDECK_MINI),
    (VENDOR_ID, PRODUCT_ID_STREAMDECK_MK2),
    (VENDOR_ID, PRODUCT_ID_STREAMDECK_ORIGINAL),
    (VENDOR_ID, PRODUCT_ID_STREAMDECK_ORIGINAL_V2),
    (VENDOR_ID, PRODUCT_ID_STREAMDECK_XL),
];

#[derive(Clone)]
pub enum StreamdeckEnum {
    StreamdeckMini(StreamdeckMini),
    StreamdeckMk2(StreamdeckMk2),
    StreamdeckOriginal(StreamdeckOriginal),
    StreamdeckOriginalV2(StreamdeckOriginalV2),
    StreamdeckXl(StreamdeckXl),
}

#[derive(Clone, serde::Serialize)]
struct StreamdeckAttached {
    serial_number: String,
}

#[derive(Clone, serde::Serialize)]
struct StreamdeckDetached {
    serial_number: String,
}

pub struct Manager {
    thread_handle: JoinHandle<i32>,
    streamdecks: Arc<Mutex<HashMap<String, StreamdeckEnum>>>
}

impl Manager {
    pub fn new(app_handle: AppHandle<Wry>) -> Self
    {        
        let streamdecks = Arc::new(Mutex::new(HashMap::new()));
        let streamdecks_arc = streamdecks.clone();
        
        let attached_app_handle = app_handle.clone();
        let detached_app_handle = app_handle.clone();

        let thread_handle = spawn(move || {
            loop {
                let mut streamdecks = streamdecks_arc.lock().unwrap();

                // Add new streamdeck
                let attached_streamdecks = Self::enumerate();
                for (serial, streamdeck) in attached_streamdecks.iter() {
                    if !streamdecks.contains_key(serial) {
                        streamdecks.insert(serial.clone(), streamdeck.clone());

                        attached_app_handle.emit_all("streamdeck_attached", StreamdeckAttached {
                            serial_number: serial.clone()
                        }).unwrap();
                    }
                }

                // Remove not connected streamdeck 
                let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
                for streamdeck_id in streamdecks.keys() {
                    if !attached_streamdecks.contains_key(streamdeck_id) {
                        to_remove.push(streamdeck_id.clone());
                    }
                }
                for streamdeck_id in to_remove {
                    streamdecks.remove(&streamdeck_id);

                    detached_app_handle.emit_all("streamdeck_detached", StreamdeckDetached {
                        serial_number: streamdeck_id.clone()
                    }).unwrap();
                }

                sleep(Duration::from_secs(1));
            }
        });
        
        Self {
            thread_handle,
            streamdecks
        }
    }

    pub fn enumerate() -> HashMap<String, StreamdeckEnum>
    {
        let mut hid_api = HidApi::new().unwrap();

        hid_api.refresh_devices().unwrap();

        let mut streamdecks = HashMap::new();

        for (vid, pid) in STREAMDECK_PRODUCTS {
            for device in hid_api.device_list() {
                if device.vendor_id() != vid || device.product_id() != pid {
                    continue;
                }
                match pid {
                    PRODUCT_ID_STREAMDECK_MINI => {
                        streamdecks.insert(
                            device.serial_number().unwrap().to_string(),
                            StreamdeckEnum::StreamdeckMini(
                                StreamdeckMini::new(
                                    device.clone()
                                )
                            )
                        );
                    }
                    PRODUCT_ID_STREAMDECK_MK2 => {
                        streamdecks.insert(
                            device.serial_number().unwrap().to_string(),
                            StreamdeckEnum::StreamdeckMk2(
                                StreamdeckMk2::new(
                                    device.clone()
                                )
                            )
                        );
                    }
                    PRODUCT_ID_STREAMDECK_ORIGINAL => {
                        streamdecks.insert(
                            device.serial_number().unwrap().to_string(),
                            StreamdeckEnum::StreamdeckOriginal(
                                StreamdeckOriginal::new(
                                    device.clone()
                                )
                            )
                        );
                    }
                    PRODUCT_ID_STREAMDECK_ORIGINAL_V2 => {
                        streamdecks.insert(
                            device.serial_number().unwrap().to_string(),
                            StreamdeckEnum::StreamdeckOriginalV2(
                                StreamdeckOriginalV2::new(
                                    device.clone()
                                )
                            )
                        );
                    }
                    PRODUCT_ID_STREAMDECK_XL => {
                        streamdecks.insert(
                            device.serial_number().unwrap().to_string(),
                            StreamdeckEnum::StreamdeckXl(
                                StreamdeckXl::new(
                                    device.clone()
                                )
                            )
                        );
                    }
                    _ => {}
                }
            }
        }

        streamdecks
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        // TODO: Join the thread
        //self.thread_handle.join().unwrap();
    }
}

