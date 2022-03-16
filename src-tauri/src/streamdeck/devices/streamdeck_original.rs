use hidapi::DeviceInfo;
use crate::streamdeck::devices::streamdeck::Streamdeck;

#[derive(Clone)]
pub struct StreamdeckOriginal {
    device: DeviceInfo
}

impl Streamdeck for StreamdeckOriginal {
    fn get_device(&self) -> &DeviceInfo {
        &self.device
    }
    fn get_columns(&self) -> i32
    {
        5
    }
    fn get_rows(&self) -> i32
    {
        3
    }
} 

impl StreamdeckOriginal {
    pub fn new(device: DeviceInfo) -> Self
    {
        Self {
            device
        }
    } 
}
