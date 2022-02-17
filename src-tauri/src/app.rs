extern crate hidapi;

use std::collections::HashMap;
use hidapi::HidApi;
use streamdeck::StreamDeck;

const ELGATO_VENDOR_ID: u16 = 0x0fd9;

pub struct App {
    pub decks: HashMap<String, StreamDeck>
}

unsafe impl Send for App {}

impl App {
    pub fn new() -> App { 
        let mut app = App {
            decks: HashMap::new()
        };
        
        app.init(HidApi::new().unwrap());
        
        app
    }

    fn init(&mut self, hid_api: HidApi) {
        self.decks.clear();

        for device in hid_api.device_list() {
            if device.vendor_id() == ELGATO_VENDOR_ID {
                if let Some(serial_number) = device.serial_number() {
                    let deck = StreamDeck::connect_with_hid(
                        &hid_api,
                        device.vendor_id(),
                        device.product_id(),
                        Some(serial_number.to_string())
                    ).expect("Unable to connect to streamdeck");
                    
                    self.decks.insert(serial_number.to_string(), deck);
                }
            }
        }
    }
    
    pub fn deck(&mut self, serial_number: String) -> Option<&mut StreamDeck> {
        self.decks.get_mut(&serial_number)
    }
}
