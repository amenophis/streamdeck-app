use std::sync::{Arc, Mutex};
use tauri::State;
use crate::{Manager};

#[tauri::command]
pub(crate) fn get_version(_app_state: State<'_, Arc<Mutex<Manager>>>, _serial: String) -> String {
    // let mut manager = _app_state
    //     .inner()
    //     .lock()
    //     .expect("Could not retrieve AppState");

    // TODO: Get the device, and call get version

    "".to_string()
}
