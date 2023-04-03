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
    pub os: Vec<OS>,
    #[serde(rename = "Software")]
    pub software: Software,
    #[serde(rename = "Category")]
    pub category: Option<String>,
    #[serde(rename = "CategoryIcon")]
    pub category_icon: Option<String>,
    #[serde(rename = "CodePathMac")]
    pub code_path_mac: Option<String>,
    #[serde(rename = "CodePathWin")]
    pub code_path_win: Option<String>,
    #[serde(rename = "CodePathLin")]
    pub code_path_lin: Option<String>,
    #[serde(rename = "Profiles")]
    pub profiles: Option<String>,
    #[serde(rename = "PropertyInspectorPath")]
    pub property_inspector_path: Option<String>,
    #[serde(rename = "DefaultWindowSize")]
    pub default_window_size: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    // #[serde(rename = "ApplicationsToMonitor")]
    // pub applications_to_monitor: Option<??>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#actions
pub struct Action {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "States")]
    pub states: Vec<State>,
    #[serde(rename = "PropertyInspectorPath")]
    pub property_inspector_path: Option<String>,
    #[serde(rename = "SupportedInMultiActions")]
    pub supported_in_multi_actions: Option<bool>,
    #[serde(rename = "Tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "DisableCaching")]
    pub disable_caching: Option<bool>,
    #[serde(rename = "VisibleInActionsList")]
    pub visible_in_actions_list: Option<bool>,
    #[serde(rename = "UserTitleEnabled")]
    pub user_title_enabled: Option<bool>,
    #[serde(rename = "Controllers")]
    pub controllers: Option<Vec<String>>,
    #[serde(rename = "Encoder")]
    pub encoder: Option<Encoder>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#states
pub struct State {
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "MultiActionImage")]
    pub multi_action_image: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "ShowTitle")]
    pub show_title: Option<String>,
    #[serde(rename = "TitleColor")]
    pub title_color: Option<String>,
    #[serde(rename = "TitleAlignment")]
    pub title_alignment: Option<String>,
    #[serde(rename = "FontFamily")]
    pub font_family: Option<String>,
    #[serde(rename = "FontStyle")]
    pub font_style: Option<String>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<String>,
    #[serde(rename = "FontUnderline")]
    pub font_underline: Option<bool>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#encoder-sd
pub struct Encoder {
    #[serde(rename = "background")]
    pub background: Option<String>,
    #[serde(rename = "Icon")]
    pub icon: Option<String>,
    #[serde(rename = "layout")]
    pub layout: Option<String>,
    #[serde(rename = "StackColor")]
    pub stack_color: Option<String>,
    #[serde(rename = "TriggerDescription")]
    pub trigger_description: Option<TriggerDescription>,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#triggerdescription-sd
pub struct TriggerDescription {
    #[serde(rename = "Rotate")]
    pub rotate: String,
    #[serde(rename = "Push")]
    pub push: String,
    #[serde(rename = "Touch")]
    pub touch: String,
    #[serde(rename = "LongTouch")]
    pub long_touch: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#profiles
pub struct Profile {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DeviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "Readonly")]
    pub readonly: Option<bool>,
    #[serde(rename = "DontAutoSwitchWhenInstalled")]
    pub dont_auto_switch_when_installed: Option<bool>,
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
    pub platform: String,
    #[serde(rename = "MinimumVersion")]
    pub minimum_version: String,
}

#[derive(Debug, Deserialize)]
// https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#software
pub struct Software {
    #[serde(rename = "MinimumVersion")]
    pub minimum_version: String,
}

// // #[derive(Debug, Deserialize)]
// // https://developer.elgato.com/documentation/stream-deck/sdk/manifest/#applicationstomonitor
// pub struct ApplicationsToMonitor {
// }
