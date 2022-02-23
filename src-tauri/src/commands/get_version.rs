use std::sync::{Arc, Mutex};
use tauri::State;
use crate::App;

#[tauri::command]
pub(crate) fn get_version(_app_state: State<'_, Arc<Mutex<App>>>, serial: String) -> String {
    let mut app = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve AppState");

    let deck = app.device_manager.device(serial).unwrap();
    
    deck.version().unwrap()
}
