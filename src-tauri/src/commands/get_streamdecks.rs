use std::{sync::{Arc, Mutex}, collections::HashMap};

use serde::Serialize;
use tauri::State;

use crate::streamdeck::manager::StreamdeckMap;


#[derive(Serialize)]
pub struct StreamDeckResponse
{
    name: String
}

#[tauri::command]
pub(crate) fn get_streamdecks(_app_state: State<'_, Arc<Mutex<StreamdeckMap>>>) -> HashMap<String, StreamDeckResponse> {
    let streamdecks = _app_state
        .inner()
        .lock()
        .expect("Could not retrieve StreamdeckMap");

    let mut streamdeck_responses = HashMap::new();

    for (serial, streamdeck) in streamdecks.iter() {
        streamdeck_responses.insert(serial.clone(), StreamDeckResponse {
            name: streamdeck.get_name()
        });
    }

    streamdeck_responses
}
