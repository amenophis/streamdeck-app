#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Module declarations
mod streamdeck {
    pub(crate) mod manager;

    pub(crate) mod devices {
        pub(crate) mod streamdeck;
        pub(crate) mod streamdeck_mini;
        pub(crate) mod streamdeck_mk2;
        pub(crate) mod streamdeck_original;
        pub(crate) mod streamdeck_original_v2;
        pub(crate) mod streamdeck_xl;
    }
}

mod commands {
    pub(crate) mod get_streamdecks;
    pub(crate) mod get_version;
}

use std::{sync::{Arc, Mutex}};

use hidapi::HidApi;
use streamdeck::manager::StreamdeckMap;

// Imports
use crate::streamdeck::manager::Manager;

use tauri::Manager as TauriManager;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();

            app.manage(Arc::new(Mutex::new(StreamdeckMap::new())));
            app.manage(Arc::new(Mutex::new(HidApi::new().unwrap())));
            app.manage(Arc::new(Mutex::new(Manager::new(app_handle))));
            
//             let mut hid_api_arc = app.state::<Arc<Mutex<HidApi>>>();
//             let mut hid_api = hid_api_arc.lock().unwrap();
//             hid_api.refresh_devices();

// println!("bjr");

//             for d in hid_api.device_list() {
//                 println!("{}", d.serial_number().unwrap());
//             }

            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_streamdecks::get_streamdecks,
            crate::commands::get_version::get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}  

