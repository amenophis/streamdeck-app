use crate::streamdeck::plugin::{
    events_received::{Coordinates, WillAppear, WillAppearPayload},
    manifest::Manifest,
    plugin_events::PluginEvent,
};
use futures_util::{SinkExt, StreamExt};
use rocket_contrib::serve::StaticFiles;
use tauri::{
    async_runtime::{spawn, Sender},
    AppHandle,
};
use tokio::{fs, net::TcpListener};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use self::info::{Application, Info};

mod events_received;
mod info;
mod manifest;
pub mod plugin_events;

pub(crate) struct Plugin {
    pub path: String,
    manifest: Manifest,
    port: Option<u16>,
}

const INIT_SCRIPT: &str = r#"
window.addEventListener("DOMContentLoaded", (event) => {
    connectElgatoStreamDeckSocket("@inPort@", "@inPluginUUID@", "@inRegisterEvent@", '@inInfo@');
});"#;

impl Plugin {
    pub async fn new(path: String) -> Self {
        let manifest_content = fs::read_to_string(path.clone() + "/manifest.json")
            .await
            .unwrap();
        let manifest = serde_json::from_str(manifest_content.as_str()).unwrap();

        Self {
            path,
            manifest,
            port: None,
        }
    }

    pub async fn start(&mut self, app_handle: AppHandle, plugin_event_sender: Sender<PluginEvent>) {
        self.start_websocket_server(plugin_event_sender).await;
        self.start_web_server().await;
        self.start_code(app_handle.clone()).await;
    }

    async fn start_websocket_server(&mut self, plugin_event_sender: Sender<PluginEvent>) {
        let addr = "127.0.0.1:0".to_string();
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);

        self.port = Some(listener.local_addr().unwrap().port());

        spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let ws_stream = tokio_tungstenite::accept_async(stream)
                    .await
                    .expect("Error during the websocket handshake occurred");

                let (mut write, mut read) = ws_stream.split();

                let plugin_event_sender = plugin_event_sender.clone();
                spawn(async move {
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                let json = &msg.into_text().unwrap().clone();
                                let event: PluginEvent = serde_json::from_str(json).unwrap();

                                match plugin_event_sender.send(event).await {
                                    Ok(_) => {
                                        dbg!("OK");
                                        ()
                                    }
                                    Err(error) => {
                                        dbg!(error);
                                        ()
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                });

                let json = serde_json::to_string(&WillAppear {
                    action: "com.elgato.sample-clock.action".to_string(),
                    event: "willAppear".to_string(),
                    context: "1".to_string(),
                    device: "a".to_string(),
                    payload: WillAppearPayload {
                        settings: "{}".to_string(),
                        coordinates: Coordinates { column: 0, row: 0 },
                        state: 0,
                        is_in_multi_action: false,
                        controller: "Keypad".to_string(),
                    },
                })
                .unwrap();

                write
                    .send(Message::Text(json.clone().to_string()))
                    .await
                    .unwrap();
            }
        });
    }

    async fn start_web_server(&self) {
        let path = self.path.clone();

        spawn(async move {
            rocket::ignite()
                .mount("/", StaticFiles::from(path))
                .launch();
        });
    }

    async fn start_code(&self, app_handle: AppHandle) {
        let path = self.get_code_path();

        let port = self.port.unwrap();

        spawn(async move {
            let info = Info {
                application: Application {
                    language: "en".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            };

            let init_script = INIT_SCRIPT
                .replace("@inPort@", port.to_string().as_str())
                .replace("@inPluginUUID@", "{}")
                .replace("@inRegisterEvent@", "registerPluginEvent")
                .replace("@inInfo@", &*serde_json::to_string(&info).unwrap());
            let window = tauri::WindowBuilder::new(
                &app_handle,
                "local",
                tauri::WindowUrl::External(Url::parse(&path).unwrap()),
            )
            //.visible(false)
            .initialization_script(&init_script)
            .build()
            .unwrap();

            window.open_devtools();
        });
    }

    fn get_code_path(&self) -> String {
        let path = "http://localhost:8000/".to_string();

        if let Some(code_path) = &self.manifest.code_path_lin {
            path + code_path
        } else if let Some(code_path) = &self.manifest.code_path_mac {
            path + code_path
        } else if let Some(code_path) = &self.manifest.code_path_win {
            path + code_path
        } else {
            path + &self.manifest.code_path
        }
    }
}
