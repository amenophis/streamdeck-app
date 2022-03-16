use hidapi::DeviceInfo;
use crate::streamdeck::devices::streamdeck::Streamdeck;

#[derive(Clone)]
pub struct StreamdeckMini {
    device: DeviceInfo
}

impl Streamdeck for StreamdeckMini {
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

impl StreamdeckMini {
    pub fn new(device: DeviceInfo) -> Self
    {
        Self {
            device
        }
    } 
}
