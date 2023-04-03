#![allow(dead_code)]

use serde::Serialize;

#[derive(Default, Debug, Serialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/registration-procedure/#info-parameter
pub struct Info {
    pub application: Application,
    pub plugin: Plugin,
    pub devices: Vec<Device>,
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: String,
    pub colors: Colors,
}

#[derive(Default, Debug, Serialize)]
pub enum Platform {
    Mac,
    Windows,
    #[default]
    Linux,
}

impl Platform {
    fn as_str(&self) -> &'static str {
        match self {
            Platform::Mac => "mac",
            Platform::Windows => "windows",
            Platform::Linux => "linux",
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Application {
    pub language: String,
    pub platform: Platform,
    pub version: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
}

#[derive(Default, Debug, Serialize)]
pub struct Plugin {
    pub version: String,
    pub uuid: String,
}

#[derive(Default, Debug, Serialize)]
pub enum DeviceType {
    #[default]
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    CorsairGKeys = 4,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7,
}

#[derive(Default, Debug, Serialize)]
pub struct DeviceSize {
    pub columns: u8,
    pub rows: u8,
}

#[derive(Default, Debug, Serialize)]
pub struct Device {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: DeviceType,
    pub size: DeviceSize,
    pub name: String,
}

#[derive(Default, Debug, Serialize)]
pub struct Colors {}
