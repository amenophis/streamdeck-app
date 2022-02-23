#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use crate::app::App;

mod app;
mod devices {
    pub(crate) mod device_manager;
    pub(crate) mod device;
    pub(crate) mod elgato {
        pub(crate) mod streamdeck;
    }
}
mod commands {
    pub(crate) mod get_devices;
    pub(crate) mod get_version;
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(App::new())))
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_devices::get_devices,
            crate::commands::get_version::get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

