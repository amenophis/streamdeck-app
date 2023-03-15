#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EventTarget {
    HardwareAndSoftware = 0,
    Hardware = 1,
    Software = 2,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setsettings
pub struct SetSettings {
    event: String,
    context: String,
    payload: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#getsettings
pub struct GetSettings {
    event: String,
    context: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setglobalsettings
pub struct SetGlobalSettings {
    event: String,
    context: String,
    payload: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#getglobalsettings
pub struct GetGlobalSettings {
    event: String,
    context: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#openurl
pub struct OpenUrl {
    event: String,
    payload: OpenUrlPayload,
}

#[derive(Debug, Deserialize)]
pub struct OpenUrlPayload {
    url: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#logmessage
pub struct LogMessage {
    event: String,
    payload: LogMessagePayload,
}

#[derive(Debug, Deserialize)]
pub struct LogMessagePayload {
    message: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#settitle
pub struct SetTitle {
    event: String,
    context: String,
    payload: SetTitlePayload,
}

#[derive(Debug, Deserialize)]
pub struct SetTitlePayload {
    title: String,
    target: EventTarget,
    state: i8,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setimage
pub struct SetImage {
    event: String,
    context: String,
    payload: SetImagePayload,
}

#[derive(Debug, Deserialize)]
pub struct SetImagePayload {
    image: String,
    target: EventTarget,
    state: i8,
}

// #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setfeedback
// pub struct SetFeedback {}

// #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setfeedbacklayout
// pub struct SetFeedbackLayout {}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#showalert
pub struct ShowAlert {
    event: String,
    context: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#showok
pub struct ShowOk {
    event: String,
    context: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setstate
pub struct SetState {
    event: String,
    context: String,
    payload: SetStatePayload,
}

#[derive(Debug, Deserialize)]
pub struct SetStatePayload {
    state: i8,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#switchtoprofile
pub struct SwitchToProfile {
    event: String,
    context: String,
    device: String,
    payload: SwitchToProfilePayload,
}

#[derive(Debug, Deserialize)]
pub struct SwitchToProfilePayload {
    profile: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#sendtopropertyinspector
pub struct SendToPropertyInspector {
    action: String,
    event: String,
    context: String,
    payload: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#sendtoplugin
pub struct SendToPlugin {
    action: String,
    event: String,
    context: String,
}
