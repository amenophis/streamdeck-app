use hidapi::DeviceInfo;

pub trait Streamdeck: Send
{
    fn get_device(&self) -> &DeviceInfo;
    fn serial_number(&self) -> String
    {
        self.get_device().serial_number().unwrap().to_string()
    }
    
    fn get_columns(&self) -> i32;
    fn get_rows(&self) -> i32;
}

// pub struct Streamdeck {
//     hid_api: Arc<Mutex<HidApi>>,c
//     device_info: DeviceInfo,
// }
// 
// impl Streamdeck {
//     pub fn new(hid_api: Arc<Mutex<HidApi>>, device_info: DeviceInfo) -> Self {
//         Self {
//             hid_api,
//             device_info,
//             
//         }
//     }
//     
//     pub fn open(&mut self) {
//         // if (self.)
//         match self.hid_api.lock() {
//             Ok(hid_api) => {
//                 match self.device_info.open_device(&hid_api) {
//                     Ok(_) => { }
//                     Err(_) => { }
//                 }
//             },
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//     }
// }

// trait Streamdeck {
//     fn open(&self);
// }

// trait SetBrightness<T: Streamdeck> {
//     fn set_brightness(&self);
// }

// impl SetBrightness<T> for Streamdeck {
//     fn set_brightness(&self) {
//         println!("Set Brightness");
//         match self.hid_api.lock() {
//             Ok(hid_api) => {
//                 match hid_api.open_serial(
//                     self.device_info.vendor_id(),
//                     self.device_info.product_id(),
//                     self.device_info.serial_number().unwrap()
//                 ) {
//                     Ok(device) => {
//                         for brightness in 0..=100 {
//                             println!("{}", brightness);
//                             let mut cmd = [0u8; 17];
//                             cmd[..3].copy_from_slice(&[0x03, 0x08, brightness]);

//                             device.send_feature_report(&cmd).unwrap();

//                             sleep(Duration::from_millis(100));
//                         }
//                     },
//                     Err(e) => {
//                         println!("Error: {}", e);

//                     }
//                 }
//             },
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//     }
// }
