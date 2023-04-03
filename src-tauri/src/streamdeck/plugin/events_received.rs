#![allow(dead_code)]

use serde::Serialize;

// pub struct ReceivedEvent {
//     action: String,
//     event: String,
//     context: String,
//     device: String,
//     payload: String,
// }

// impl ReceivedEvent {
//     // https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#didreceivesettings
//     pub fn didReceiveSettings(
//         action: String,
//         context: String,
//         device: String,
//         payload: DidReceiveSettingsPayload,
//     ) -> Self {
//         Self {
//             action,
//             event: "didReceiveSettings".into(),
//             context,
//             device,
//             payload: payload.into(),
//         }
//     }
// }

// impl<'de> Deserialize<'de> for ReceivedEvent {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.dese
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct DidReceiveSettingsPayload {
//     settings: String,
//     coordinates: String,
//     state: String,
//     is_in_multi_action: bool,
// }

// #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#didreceiveglobalsettings
// pub struct DidReceiveGlobalSettings {
//     event: String,
//     payload: DidReceiveSettingsPayload,
// }

// #[derive(Debug, Deserialize)]
// pub struct DidReceiveGlobalSettingsPayload {
//     settings: String,
// }

// #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#keydown
// pub struct KeyDown {
//     action: String,
//     event: String,
//     context: String,
//     device: String,
//     payload: DidReceiveSettingsPayload,
// }

// #[derive(Debug, Deserialize)]
// pub struct KeyDownPayload {
//     settings: String,
// }

#[derive(Debug, Serialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#willappear
pub struct WillAppear {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: WillAppearPayload,
}

#[derive(Debug, Serialize)]
pub struct WillAppearPayload {
    pub settings: String,
    pub coordinates: Coordinates,
    pub state: u8,
    #[serde(rename = "isInMultiAction")]
    pub is_in_multi_action: bool,
    pub controller: String,
}

#[derive(Debug, Serialize)]
pub struct Coordinates {
    pub column: u8,
    pub row: u8,
}
