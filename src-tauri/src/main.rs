#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use hidapi::HidApi;
use std::sync::Arc;

use streamdeck::manager::{Manager, StreamdeckMap};
use tauri::{async_runtime::Mutex, Manager as TauriManager};

mod streamdeck {
    pub(crate) mod manager;
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            println!("Starting ...");

            let hid_api = HidApi::new();
            if hid_api.is_err() {
                panic!("Unable to open hid_api");
            }

            let hid_api_mutex = Arc::new(Mutex::new(hid_api.unwrap()));
            app.manage(hid_api_mutex.clone());

            let streamdeck_map_mutex = Arc::new(Mutex::new(StreamdeckMap::new()));
            app.manage(streamdeck_map_mutex.clone());

            app.manage(Manager::new(
                hid_api_mutex.clone(),
                streamdeck_map_mutex.clone(),
                app,
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
