#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use streamdeck::manager::DeviceManager;
use tauri::{async_runtime::block_on, Manager as TauriManager};

use crate::streamdeck::{server::StreamDeckServer, transport::TransportType};

mod streamdeck {
    pub(crate) mod manager;
    pub(crate) mod monitor;
    pub(crate) mod server;
    pub(crate) mod transport;
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            block_on(async {
                let mut stream_deck_server = StreamDeckServer::new();
                let device_manager = DeviceManager::new(TransportType::StreamdeckRs());

                stream_deck_server.start(device_manager).await;

                app.manage(stream_deck_server);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
