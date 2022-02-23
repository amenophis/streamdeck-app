extern crate hidapi;

use crate::devices::device_manager::DeviceManager;

pub struct App {
    pub device_manager: DeviceManager
}

unsafe impl Send for App {}

impl App {
    pub fn new() -> App { 
        App {
            device_manager: DeviceManager::new()
        }
    }
}
