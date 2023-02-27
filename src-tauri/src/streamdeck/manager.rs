use elgato_streamdeck::{info::Kind, list_devices, AsyncStreamDeck};
use hidapi::HidApi;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tauri::{
    async_runtime::{spawn, Mutex},
    App, Manager as TauriManager,
};

#[derive(Clone, serde::Serialize)]
struct StreamdeckPayload {
    serial: String,
    name: String,
    row_count: u8,
    column_count: u8,
    key_count: u8,
    kind: String,
}

pub struct Manager {}

pub type StreamdeckMap = HashMap<String, Arc<AsyncStreamDeck>>;

impl Manager {
    pub fn new(
        hid_api: Arc<Mutex<HidApi>>,
        streamdecks: Arc<Mutex<StreamdeckMap>>,
        app: &App,
    ) -> Self {
        let hid_api = hid_api.clone();
        let streamdecks = streamdecks.clone();

        let app_handle = app.handle();

        spawn(async move {
            println!("Starting loop");
            loop {
                let mut hid_api = hid_api.lock().await;

                let mut streamdecks = streamdecks.lock().await;

                let _ = hid_api.refresh_devices();

                let attached_streamdecks = list_devices(&hid_api);
                let mut attached_serials = Vec::new();

                // Add new streamdeck
                for (kind, serial) in attached_streamdecks {
                    attached_serials.push(serial.clone());

                    if streamdecks.contains_key(&serial) {
                        continue;
                    }

                    if let Ok(device) = AsyncStreamDeck::connect(&hid_api, kind, &serial) {
                        streamdecks.insert(serial.clone(), device);
                        let _ = app_handle.emit_all(
                            "device_attached",
                            StreamdeckPayload {
                                serial: serial.clone(),
                                name: match kind {
                                    Kind::Original => "Stream Deck".to_string(),
                                    Kind::OriginalV2 => "Stream Deck V2".to_string(),
                                    Kind::Mini => "Stream Deck Mini".to_string(),
                                    Kind::Xl => "Stream Deck XL".to_string(),
                                    Kind::XlV2 => "Stream Deck XL V2".to_string(),
                                    Kind::Mk2 => "Stream Deck Mk2".to_string(),
                                    Kind::MiniMk2 => "Stream Mini Deck Mk2".to_string(),
                                    Kind::Pedal => "Stream Deck Pedal".to_string(),
                                },
                                row_count: kind.row_count(),
                                column_count: kind.column_count(),
                                key_count: kind.key_count(),
                                kind: match kind {
                                    Kind::Original => "original".to_string(),
                                    Kind::OriginalV2 => "original_v2".to_string(),
                                    Kind::Mini => "mini".to_string(),
                                    Kind::Xl => "xl".to_string(),
                                    Kind::XlV2 => "xl_v2".to_string(),
                                    Kind::Mk2 => "mk2".to_string(),
                                    Kind::MiniMk2 => "mini_mk2".to_string(),
                                    Kind::Pedal => "pedal".to_string(),
                                },
                            },
                        );
                        println!("Attached {}", serial);
                    }
                }

                // Remove not connected streamdeck
                let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
                for serial in streamdecks.keys() {
                    if !attached_serials.contains(&serial) {
                        to_remove.push(serial.clone());
                    }
                }
                for serial in to_remove {
                    streamdecks.remove(&serial);
                    let _ = app_handle.emit_all("device_detached", serial.clone());
                    println!("Detached {}", serial);
                }

                sleep(Duration::from_millis(100));
            }
        });

        Self {}
    }
}
