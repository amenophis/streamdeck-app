use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum PluginEvent {
    // ??
    #[serde(rename = "registerPluginEvent")]
    RegisterPluginEvent {},
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setsettings
    #[serde(rename = "setSettings")]
    SetSettings {
        context: String,
        // #[serde(borrow)]
        // payload: &'a RawValue,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#getsettings
    #[serde(rename = "getSettings")]
    GetSettings { context: String },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setglobalsettings
    #[serde(rename = "setGlobalSettings")]
    SetGlobalSettings { context: String, payload: String },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#getglobalsettings
    #[serde(rename = "getGlobalSettings")]
    GetGlobalSettings { context: String },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#openurl
    #[serde(rename = "openUrl")]
    OpenUrl { payload: OpenUrlPayload },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#logmessage
    #[serde(rename = "logMessage")]
    LogMessage { payload: LogMessagePayload },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#settitle
    #[serde(rename = "setTitle")]
    SetTitle {
        context: String,
        payload: SetTitlePayload,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setimage
    #[serde(rename = "setImage")]
    SetImage {
        context: String,
        payload: SetImagePayload,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setfeedback
    #[serde(rename = "setFeedback")]
    SetFeedback {},
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setfeedbacklayout
    #[serde(rename = "setFeedbackLayout")]
    SetFeedbackLayout {},
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#showalert
    #[serde(rename = "showAlert")]
    ShowAlert { context: String },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#showok
    #[serde(rename = "showOk")]
    ShowOk { context: String },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setstate
    #[serde(rename = "setState")]
    SetState {
        context: String,
        payload: SetStatePayload,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#switchtoprofile
    #[serde(rename = "switchToProfile")]
    SwitchToProfile {
        context: String,
        device: String,
        payload: SwitchToProfilePayload,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#sendtopropertyinspector
    #[serde(rename = "sendToPropertyInspector")]
    SendToPropertyInspector {
        action: String,
        context: String,
        payload: String,
    },
    // https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#sendtoplugin
    #[serde(rename = "sendToPlugin")]
    SendToPlugin { action: String, context: String },
}

#[derive(Debug, Deserialize)]
pub enum EventTarget {
    HardwareAndSoftware = 0,
    Hardware = 1,
    Software = 2,
}

#[derive(Debug, Deserialize)]
pub struct OpenUrlPayload {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LogMessagePayload {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SetTitlePayload {
    pub title: String,
    pub target: EventTarget,
    pub state: i8,
}

#[derive(Debug, Deserialize)]
pub struct SetImagePayload {
    pub image: String,
    // pub target: EventTarget,
    pub state: Option<i8>,
}

#[derive(Debug, Deserialize)]
pub struct SetStatePayload {
    pub state: i8,
}

#[derive(Debug, Deserialize)]
pub struct SwitchToProfilePayload {
    pub profile: String,
}
