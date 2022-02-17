#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use elgato_streamdeck::{list_devices, StreamDeck};
use hidapi::{HidApi, HidResult};


mod streamdeck {
    pub(crate) mod streamdeck;
}

#[tauri::command]
fn cmd_list_devices(
    hid: tauri::State<HidResult<HidApi>>,
) -> Vec<String>
{
    let hid = hid.as_ref().unwrap();
    
    list_devices(&hid)
        .into_iter()
        .map(|(_, serial)| serial)
        .collect()
    
}

#[tauri::command]
fn cmd_get_firmware(
    serial: String,
    hid: tauri::State<HidResult<HidApi>>,
) -> String
{
    let hid = hid.as_ref().unwrap();
    let device = StreamDeck::connect(hid, elgato_streamdeck::info::Kind::OriginalV2, &serial)
        .expect("Failed to connect");

    device.firmware_version().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd_list_devices, cmd_get_firmware])
        .manage(HidApi::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
