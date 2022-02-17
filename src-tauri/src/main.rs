#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::State;
use std::sync::{Arc, Mutex};
use streamdeck::{Kind};

mod app;

use crate::app::App;

#[tauri::command]
fn get_decks(_app_state: State<'_, Arc<Mutex<App>>>) -> Result<Vec<DeckListItem>, String> {
    let state = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve AppState");

    let mut items = Vec::new();

    for (serial_number, device) in &state.decks {
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

#[tauri::command]
fn get_version(_app_state: State<'_, Arc<Mutex<App>>>, serial: String) -> String {
    let mut app = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve AppState");

    let deck = app.deck(serial).unwrap();
    let version = deck.version().unwrap();
    
    version
}

#[derive(serde::Serialize)]
pub struct DeckListItem {
    serial: String,
    kind: String
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(App::new())))
        .invoke_handler(tauri::generate_handler![
            get_decks,
            get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
