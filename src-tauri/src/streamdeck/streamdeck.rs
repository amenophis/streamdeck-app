// use std::collections::HashMap;
// use std::ops::DerefMut;
// use std::sync::{Arc, Mutex};
// use std::thread::{sleep, spawn};
// use std::time::Duration;
// use hidapi::{HidApi, HidDevice};

// const VENDOR_ID: u16 = 0x0fd9;

// const PRODUCT_ID_STREAMDECK_MINI: u16 = 0x0063;
// const PRODUCT_ID_STREAMDECK_MK2: u16 = 0x0080;
// const PRODUCT_ID_STREAMDECK_ORIGINAL: u16 = 0x0060;
// const PRODUCT_ID_STREAMDECK_ORIGINAL_V2: u16 = 0x006d;
// const PRODUCT_ID_STREAMDECK_XL: u16 = 0x006c;

// static STREAMDECK_PRODUCTS: [u16; 5] = [
//     PRODUCT_ID_STREAMDECK_MINI,
//     PRODUCT_ID_STREAMDECK_MK2,
//     PRODUCT_ID_STREAMDECK_ORIGINAL,
//     PRODUCT_ID_STREAMDECK_ORIGINAL_V2,
//     PRODUCT_ID_STREAMDECK_XL,
// ];

// pub struct Manager {
//     hid_api: Arc<Mutex<HidApi>>,
//     streamdecks: Arc<Mutex<HashMap<String, HidDevice>>>,
// }

// pub type StreamdeckMap = HashMap<String, HidDevice>;

// impl Manager {
//     pub fn new() -> Self
//     {
//         // let hid_api = Arc::new(Mutex::new(HidApi::new().unwrap()));
//         // let streamdecks = Arc::new(Mutex::new(StreamdeckMap::new()));

//         // let hid_api_ref = hid_api.clone();
//         // let streamdecks_ref = streamdecks.clone();

//         // spawn(move || {
//         //     loop {
//         //         let mut hid_api_ref = hid_api_ref.lock().unwrap();
//         //         let hid_api_ref = hid_api_ref.deref_mut();
//         //         hid_api_ref.refresh_devices().unwrap();

//         //         let mut streamdecks_ref = streamdecks_ref.lock().unwrap();
//         //         let streamdecks_ref = streamdecks_ref.deref_mut();

//         //         // Add new streamdeck
//         //         let mut attached_streamdecks= HashMap::new();

//         //         for device in hid_api_ref.device_list() {
//         //             if device.vendor_id() != VENDOR_ID || !STREAMDECK_PRODUCTS.contains(&device.product_id()) {
//         //                 continue;
//         //             }
            
//         //             let serial = device.serial_number().unwrap().to_string();
            
//         //             attached_streamdecks.insert(serial, device.clone());
//         //         }

//         //         for (serial, device) in attached_streamdecks.iter() {
//         //             let s = serial.clone(); 

//         //             if streamdecks_ref.contains_key(&s) {
//         //                 continue;
//         //             }

//         //             let opened_device = device.open_device(&hid_api_ref).unwrap();
                    
//         //             let streamdeck = match device.product_id() {
//         //                 PRODUCT_ID_STREAMDECK_MINI | PRODUCT_ID_STREAMDECK_MK2 | PRODUCT_ID_STREAMDECK_ORIGINAL | PRODUCT_ID_STREAMDECK_ORIGINAL_V2 | PRODUCT_ID_STREAMDECK_XL  => {
//         //                     Some(opened_device)
//         //                 }
//         //                 _ => None
//         //             };

//         //             if streamdeck.is_some() {
//         //                 streamdecks_ref.insert(serial.clone(), streamdeck.unwrap());
//         //                 println!("Attached {}", serial);
//         //             }
//         //         }

//         //         // Remove not connected streamdeck 
//         //         let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
//         //         for serial in streamdecks_ref.keys() {
//         //             if !attached_streamdecks.contains_key(serial) {
//         //                 to_remove.push(serial.clone());
//         //             }
//         //         }
//         //         for serial in to_remove {
//         //             streamdecks_ref.remove(&serial);
                
//         //             println!("Detached {}", serial);
//         //         }

//         //         sleep(Duration::from_secs(1));
//         //     }
//         // });

//         Self {
//             hid_api: hid_api,
//             streamdecks: streamdecks,
//         }
//     }

//     pub fn list_devices(&self) -> Vec<String> 
//     {
//         let mut streamdecks = self.streamdecks.lock().unwrap();
//         let streamdecks = streamdecks.deref_mut();

//         let mut serials = Vec::new();

//         for (serial, _) in streamdecks.iter() {
//             serials.push(serial.to_string());
//         }

//         serials
//     }

//     pub fn get_firmware(&self) -> Vec<String> 
//     {
//         let mut streamdecks = self.streamdecks.lock().unwrap();
//         let streamdecks = streamdecks.deref_mut();

//         let mut serials = Vec::new();

//         for (serial, _) in streamdecks.iter() {
//             serials.push(serial.to_string());
//         }

//         serials
//     }
// }
