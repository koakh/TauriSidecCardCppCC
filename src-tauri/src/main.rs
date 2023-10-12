// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];
#[cfg(debug_assertions)]
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
const LOG_FILTER_MODULE: &str = "tauri_sidecard_cpp";

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];
#[cfg(not(debug_assertions))]
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Warn;
#[cfg(not(debug_assertions))]
const LOG_FILTER_MODULE: &str = "tauri_sidecard_cpp";

use log::{info, LevelFilter};
use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    info!("tauri: greet arg: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(debug_assertions)]
            window.open_devtools();

            tauri::async_runtime::spawn(async move {
                let (mut rx, mut child) = Command::new_sidecar("app")
                    .expect("failed to setup `app` sidecar")
                    .spawn()
                    .expect("Failed to spawn packaged node");

                let mut i = 0;
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        window
                            .emit("message", Some(format!("'{}'", line)))
                            .expect("failed to emit event");
                        info!("tauri: sidecar stdout: {}", line);
                        i += 1;
                        if i == 4 {
                            child.write("message from Rust".as_bytes()).unwrap();
                            i = 0;
                        }
                    }
                }
            });

            Ok(())
        })
        // tauri log plugin
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                // .level(LOG_LEVEL_FILTER)
                .level_for(LOG_FILTER_MODULE, LOG_LEVEL_FILTER)
                .build(),
        ) // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
