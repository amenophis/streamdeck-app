use crate::streamdeck::plugin::manifest::Manifest;
use futures_util::{future, StreamExt, TryStreamExt};
use std::collections::HashMap;
use tauri::{async_runtime::spawn, AppHandle};
use tokio::{fs, net::TcpListener};
use url::Url;

mod manifest;

static PLUGINS_PATH: &str = "/home/jeremy/.config/streamdeck-app/plugins/";

#[derive(Debug)]
pub(crate) struct Plugin {
    path: String,
    manifest: Manifest,
}

const INIT_SCRIPT: &str = r#"
window.addEventListener("DOMContentLoaded", (event) => {
    connectElgatoStreamDeckSocket(#port#, 8080, 8080, 8080, 8080);
});"#;

impl Plugin {
    pub async fn new(path: String) -> Self {
        let manifest_content = fs::read_to_string(path.clone() + "/manifest.json")
            .await
            .unwrap();
        let manifest = serde_json::from_str(manifest_content.as_str()).unwrap();

        Self { path, manifest }
    }

    pub async fn start(&self, app_handle: AppHandle) {
        self.start_websocket().await;
        self.start_code(app_handle.clone()).await;
    }

    async fn start_websocket(&self) {
        let addr = "127.0.0.1:8080".to_string();
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);

        spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                spawn(async move {
                    let ws_stream = tokio_tungstenite::accept_async(stream)
                        .await
                        .expect("Error during the websocket handshake occurred");

                    let (write, read) = ws_stream.split();

                    // We should not forward messages other than text or binary.
                    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
                        .forward(write)
                        .await
                        .expect("Failed to forward messages")
                });
            }
        });
    }

    async fn start_code(&self, app_handle: AppHandle) {
        let path = "file://".to_string()
            + &self.path.clone().to_string()
            + "/"
            + &self.manifest.code_path.clone().to_string();

        spawn(async move {
            let init_script = INIT_SCRIPT.replace(&"#port#".to_string(), &"8080".to_string());
            let local_window = tauri::WindowBuilder::new(
                &app_handle,
                "local",
                tauri::WindowUrl::External(Url::parse(&path).unwrap()),
            )
            .initialization_script(&init_script)
            .build()
            .unwrap();

            local_window.open_devtools();
        });
    }
}

pub(crate) struct PluginManager {
    app_handle: AppHandle,
    plugins: HashMap<String, Plugin>,
}

impl PluginManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            plugins: HashMap::new(),
        }
    }

    pub async fn start(&mut self) {
        self.plugins = self.list_plugins().await;

        for (_name, plugin) in self.plugins.iter() {
            plugin.start(self.app_handle.clone()).await;
        }
    }

    pub async fn list_plugins(&self) -> HashMap<String, Plugin> {
        let mut plugins = HashMap::new();
        let mut reader = fs::read_dir(PLUGINS_PATH).await.unwrap();
        loop {
            if let Some(f) = reader.next_entry().await.unwrap() {
                let plugin_name = f.file_name().to_str().unwrap().to_string();
                let path = f.path().to_str().unwrap().to_string();
                plugins.insert(plugin_name, Plugin::new(path).await);
            } else {
                break;
            }
        }

        plugins
    }
}
