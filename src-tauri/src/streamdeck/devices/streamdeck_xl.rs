use hidapi::DeviceInfo;
use crate::streamdeck::devices::streamdeck::Streamdeck;

#[derive(Clone)]
pub struct StreamdeckXl {
    device: DeviceInfo
}

impl Streamdeck for StreamdeckXl {
    fn get_device(&self) -> &DeviceInfo {
        &self.device
    }
    fn get_columns(&self) -> i32
    {
        8
    }
    fn get_rows(&self) -> i32
    {
        4
    }
} 

impl StreamdeckXl {
    pub fn new(device: DeviceInfo) -> Self
    {
        Self {
            device
        }
    } 
}
