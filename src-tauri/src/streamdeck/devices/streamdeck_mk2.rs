use hidapi::DeviceInfo;
use crate::streamdeck::devices::streamdeck::Streamdeck;

#[derive(Clone)]
pub struct StreamdeckMk2 {
    device: DeviceInfo
}

impl Streamdeck for StreamdeckMk2 {
    fn get_device(&self) -> &DeviceInfo {
        &self.device
    }
    fn get_columns(&self) -> i32
    {
        3
    }
    fn get_rows(&self) -> i32
    {
        2
    }
} 

impl StreamdeckMk2 {
    pub fn new(device: DeviceInfo) -> Self
    {
        Self {
            device
        }
    } 
}
