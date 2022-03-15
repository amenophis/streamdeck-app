#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Module declarations
mod streamdeck {
    pub(crate) mod manager;

    pub(crate) mod devices {
        pub(crate) mod streamdeck;
        pub(crate) mod streamdeck_original_v2;
    }
}

mod commands {
    pub(crate) mod get_version;
}

// Imports
use std::sync::{Arc, Mutex};
use tauri::Manager as TauriManager;
use crate::streamdeck::manager::Manager;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
             
            app.manage(
            Arc::new(
                Mutex::new(
                    Manager::new(app_handle)
                    )
                )
            );
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_version::get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}  

