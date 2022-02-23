use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use streamdeck::StreamDeck; 

const ELGATO_VENDOR_ID: u16 = 0x0fd9;

use rusb::{Context, Device, HotplugBuilder, UsbContext};
 
struct HotPlugHandler;

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        if device.device_descriptor().unwrap().vendor_id() != ELGATO_VENDOR_ID {
            return;
        }
        //self.
        println!("device arrived {:?}", device);
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("device left {:?}", device);
    }
}

impl Drop for HotPlugHandler {
    fn drop(&mut self) {
        println!("HotPlugHandler dropped");
    }
}

pub struct DeviceManager {
    need_stop: bool,
    pub devices: HashMap<String, StreamDeck>
}

impl Drop for DeviceManager
{
    fn drop(&mut self) {
        self.need_stop = true;
        println!("DeviceManager dropped");
    }
}

impl DeviceManager {
    pub fn new() -> Self {
        let s = Self {
            need_stop: false,
            devices: HashMap::new()
        };
        
        s.start();
        
        s
    }
    
    fn start(&self) {
        if rusb::has_hotplug() {
            thread::spawn(|| {
                let context = Context::new().unwrap();

                let mut reg = Some(
                    HotplugBuilder::new()
                    .enumerate(true)
                    .vendor_id(ELGATO_VENDOR_ID)
                    .register::<Context, &Context>(&context, Box::new(HotPlugHandler {}))
                );
                loop {
                    context.handle_events(None).unwrap();
                }
            });
        } else {
            panic!("libusb hotplug api unsupported");
        }
    }
    
    pub fn device(&mut self, serial_number: String) -> Option<&mut StreamDeck> {
        self.devices.get_mut(&serial_number)
    }
}
