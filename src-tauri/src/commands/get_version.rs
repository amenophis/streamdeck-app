use std::sync::{Arc, Mutex};
use tauri::State;
use crate::streamdeck::manager::StreamdeckMap;

#[tauri::command]
pub(crate) fn get_version(_app_state: State<'_, Arc<Mutex<StreamdeckMap>>>, serial: String) -> String {
    let mut streamdecks = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve StreamdeckMap");

    match streamdecks.get(&serial) {
        Some(streamdeck) => streamdeck.get_version(),
        None => "NA".into(),
    }
}
