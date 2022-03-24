use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use hidapi::{HidApi, DeviceInfo};
use tauri::{AppHandle, Manager as TauriManager, Wry};
use crate::streamdeck::devices::streamdeck::{Streamdeck, StreamdeckStruct};

const VENDOR_ID: u16 = 0x0fd9;

const PRODUCT_ID_STREAMDECK_MINI: u16 = 0x0063;
const PRODUCT_ID_STREAMDECK_MK2: u16 = 0x0080;
const PRODUCT_ID_STREAMDECK_ORIGINAL: u16 = 0x0060;
const PRODUCT_ID_STREAMDECK_ORIGINAL_V2: u16 = 0x006d;
const PRODUCT_ID_STREAMDECK_XL: u16 = 0x006c;

static STREAMDECK_PRODUCTS: [u16; 5] = [
    PRODUCT_ID_STREAMDECK_MINI,
    PRODUCT_ID_STREAMDECK_MK2,
    PRODUCT_ID_STREAMDECK_ORIGINAL,
    PRODUCT_ID_STREAMDECK_ORIGINAL_V2,
    PRODUCT_ID_STREAMDECK_XL,
];

pub struct Manager {
}

pub type StreamdeckMap = HashMap<String, Streamdeck>;

impl Manager {
    pub fn new(app: AppHandle<Wry>) -> Self
    {
        spawn(move || {
            loop {
                let mut hid_api_arc = app.state::<Arc<Mutex<HidApi>>>();
                let mut hid_api = hid_api_arc.lock().unwrap();
                let mut hid_api = hid_api.deref_mut();
                hid_api.refresh_devices().unwrap();


                let mut streamdecks_arc = app.state::<Arc<Mutex<StreamdeckMap>>>();
                let mut streamdecks = streamdecks_arc.lock().unwrap();

                // Add new streamdeck
                let mut attached_streamdecks = Manager::enumerate(hid_api);

                for (serial, device) in attached_streamdecks.iter() {
                    let s = serial.clone(); 

                    if streamdecks.contains_key(&s) {
                        continue;
                    }
    
                    let opened_device = device.open_device(&hid_api).unwrap();

                    let streamdeck = match device.product_id() {
                        PRODUCT_ID_STREAMDECK_MINI => {
                            Some(Streamdeck::Mini {
                                s: StreamdeckStruct::new(opened_device)
                            })
                        }
                        PRODUCT_ID_STREAMDECK_MK2 => {
                            Some(Streamdeck::Mk2 {
                                s: StreamdeckStruct::new(opened_device)
                            })
                        }
                        PRODUCT_ID_STREAMDECK_ORIGINAL => {
                            Some(Streamdeck::Original { 
                                s: StreamdeckStruct::new(opened_device)
                            })
                        }
                        PRODUCT_ID_STREAMDECK_ORIGINAL_V2 => {
                            Some(Streamdeck::OriginalV2 {
                                s: StreamdeckStruct::new(opened_device)
                            })
                        }
                        PRODUCT_ID_STREAMDECK_XL => {
                            Some(Streamdeck::Xl {
                                s: StreamdeckStruct::new(opened_device)
                            })
                        }
                        _ => None
                    };

                    if streamdeck.is_some() {
                        streamdecks.insert(serial.clone(), streamdeck.unwrap());
                        println!("Attached {}", serial);
                    }
                }

                // Remove not connected streamdeck 
                let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
                for serial in streamdecks.keys() {
                    if !attached_streamdecks.contains_key(serial) {
                        to_remove.push(serial.clone());
                    }
                }
                for serial in to_remove {
                    streamdecks.remove(&serial);
                
                    println!("Detached {}", serial);
                }

                sleep(Duration::from_secs(1));
            }
        });

        Self {}
    }

    fn enumerate(hid_api: &HidApi) -> HashMap<String, DeviceInfo>
    {
        let mut streamdecks = HashMap::new();

        for device in hid_api.device_list() {
            if device.vendor_id() != VENDOR_ID || !STREAMDECK_PRODUCTS.contains(&device.product_id()) {
                continue;
            }

            let serial = device.serial_number().unwrap().to_string();

            streamdecks.insert(serial, device.clone());
        }

        streamdecks
    }
} 
