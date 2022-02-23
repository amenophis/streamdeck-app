use std::sync::{Arc, Mutex};
use streamdeck::{Kind};
use tauri::State;
use crate::App;

#[derive(serde::Serialize)]
pub(crate) struct DeckListItem {
    serial: String,
    kind: String
}

#[tauri::command]
pub(crate) fn get_devices(_app_state: State<'_, Arc<Mutex<App>>>) -> Result<Vec<DeckListItem>, String> {
    let state = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve AppState");

    let mut items = Vec::new();

    for (serial_number, device) in &state.device_manager.devices {
        items.push(DeckListItem {
            serial: serial_number.clone(),
            kind: match device.kind() {
                Kind::Original => "Original".to_string(),
                Kind::OriginalV2 => "OriginalV2".to_string(),
                Kind::Mini =>"Mini".to_string(),
                Kind::Xl =>"Xl".to_string()
            }
        });
    }

    return Ok(items);
}
