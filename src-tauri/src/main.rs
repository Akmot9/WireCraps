// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod states;
use states::SonarState;

mod commands;
use commands::{get_hostname, get_interfaces};

use tauri::{generate_context, generate_handler, Manager};
use tauri_plugin_log::{Target, TargetKind};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|app| {
            let handle = app.handle().clone();
            app.manage(SonarState::new(handle));
            Ok(())
        })
        .invoke_handler(generate_handler![get_hostname, get_interfaces])
        .run(generate_context!())
        .expect("error while running tauri application");
}
