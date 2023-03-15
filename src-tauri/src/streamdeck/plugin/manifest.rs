#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#members
pub struct Manifest {
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "CodePath")]
    pub code_path: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "SDKVersion")]
    pub sdk_version: u8,
    #[serde(rename = "OS")]
    os: Vec<OS>,
    #[serde(rename = "Software")]
    software: Software,
    #[serde(rename = "Category")]
    category: Option<String>,
    #[serde(rename = "CategoryIcon")]
    category_icon: Option<String>,
    #[serde(rename = "CodePathMac")]
    code_path_mac: Option<String>,
    #[serde(rename = "CodePathWin")]
    code_path_win: Option<String>,
    #[serde(rename = "Profiles")]
    profiles: Option<String>,
    #[serde(rename = "PropertyInspectorPath")]
    property_inspector_path: Option<String>,
    #[serde(rename = "DefaultWindowSize")]
    default_window_size: Option<String>,
    #[serde(rename = "URL")]
    url: Option<String>,
    // #[serde(rename = "ApplicationsToMonitor")]
    // applications_to_monitor: Option<??>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#actions
pub struct Action {
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "States")]
    states: Vec<State>,
    #[serde(rename = "PropertyInspectorPath")]
    property_inspector_path: Option<String>,
    #[serde(rename = "SupportedInMultiActions")]
    supported_in_multi_actions: Option<bool>,
    #[serde(rename = "Tooltip")]
    tooltip: Option<String>,
    #[serde(rename = "DisableCaching")]
    disable_caching: Option<bool>,
    #[serde(rename = "VisibleInActionsList")]
    visible_in_actions_list: Option<bool>,
    #[serde(rename = "UserTitleEnabled")]
    user_title_enabled: Option<bool>,
    #[serde(rename = "Controllers")]
    controllers: Option<Vec<String>>,
    #[serde(rename = "Encoder")]
    encoder: Option<Encoder>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#states
pub struct State {
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "MultiActionImage")]
    multi_action_image: Option<String>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "ShowTitle")]
    show_title: Option<String>,
    #[serde(rename = "TitleColor")]
    title_color: Option<String>,
    #[serde(rename = "TitleAlignment")]
    title_alignment: Option<String>,
    #[serde(rename = "FontFamily")]
    font_family: Option<String>,
    #[serde(rename = "FontStyle")]
    font_style: Option<String>,
    #[serde(rename = "FontSize")]
    font_size: Option<String>,
    #[serde(rename = "FontUnderline")]
    font_underline: Option<bool>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#encoder-sd
pub struct Encoder {
    #[serde(rename = "background")]
    background: Option<String>,
    #[serde(rename = "Icon")]
    icon: Option<String>,
    #[serde(rename = "layout")]
    layout: Option<String>,
    #[serde(rename = "StackColor")]
    stack_color: Option<String>,
    #[serde(rename = "TriggerDescription")]
    trigger_description: Option<TriggerDescription>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#triggerdescription-sd
pub struct TriggerDescription {
    #[serde(rename = "Rotate")]
    rotate: String,
    #[serde(rename = "Push")]
    push: String,
    #[serde(rename = "Touch")]
    touch: String,
    #[serde(rename = "LongTouch")]
    long_touch: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#profiles
pub struct Profile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "DeviceType")]
    device_type: DeviceType,
    #[serde(rename = "Readonly")]
    readonly: Option<bool>,
    #[serde(rename = "DontAutoSwitchWhenInstalled")]
    dont_auto_switch_when_installed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum DeviceType {
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    CorsairGKeys = 4,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#os
pub struct OS {
    #[serde(rename = "Platform")]
    platform: String,
    #[serde(rename = "MinimumVersion")]
    minimum_version: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#software
pub struct Software {
    #[serde(rename = "MinimumVersion")]
    minimum_version: String,
}

// // #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#applicationstomonitor
// pub struct ApplicationsToMonitor {
// }
