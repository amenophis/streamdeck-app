use std::str::from_utf8;

use hidapi::HidDevice;

pub struct StreamdeckStruct {
    hid_device: HidDevice
}

unsafe impl Sync for StreamdeckStruct {}

impl StreamdeckStruct {
    pub fn new(hid_device: HidDevice) -> Self 
    {
        Self {
            hid_device
        }
    }
}

pub enum Streamdeck {
    Mini { s: StreamdeckStruct },
    Mk2 { s: StreamdeckStruct },
    Original { s: StreamdeckStruct },
    OriginalV2 { s: StreamdeckStruct },
    Xl { s: StreamdeckStruct },
}

impl Streamdeck {
    pub fn get_rows(&self) -> i32
    {
        match self {
            Self::Mini { .. } => 2,
            Self::Mk2 { .. } => 3,
            Self::Original { .. } => 3,
            Self::OriginalV2 { .. } => 3,
            Self::Xl { .. } => 5
        }
    }

    pub fn get_cols(&self) -> i32
    {
        match self {
            Self::Mini { .. } => 3,
            Self::Mk2 { .. } => 5,
            Self::Original { .. } => 5,
            Self::OriginalV2 { .. } => 5,
            Self::Xl { .. } => 4
        }
    }

    
    pub fn get_name(&self) -> String
    {
        let s = match self {
            Self::Mini { s } => s,
            Self::Mk2{ s } => s,
            Self::Original{ s } => s,
            Self::OriginalV2{ s } => s,
            Self::Xl{ s } => s,
        };
        
        s.hid_device.get_product_string().unwrap().unwrap()
    }
    
    pub fn set_brightness(&self, value: u8)
    {
        let s = match self {
            Self::Mini { s } => s,
            Self::Mk2{ s } => s,
            Self::Original{ s } => s,
            Self::OriginalV2{ s } => s,
            Self::Xl{ s } => s,
        };
        
        let mut cmd = [0u8; 17];
        cmd[..3].copy_from_slice(&[0x03, 0x08, value]);

        s.hid_device.send_feature_report(&cmd).unwrap();
    }

    pub fn get_version(&self) -> String
    {
        let s = match self {
            Self::Mini { s } => s,
            Self::Mk2{ s } => s,
            Self::Original{ s } => s,
            Self::OriginalV2{ s } => s,
            Self::Xl{ s } => s,
        };
        
        let mut cmd = [0u8; 17];

        cmd[0] = 0x05;

        let _s = s.hid_device.get_feature_report(&mut cmd);

        let offset = 6;
        from_utf8(&cmd[offset..]).unwrap().to_string()
    }
}
