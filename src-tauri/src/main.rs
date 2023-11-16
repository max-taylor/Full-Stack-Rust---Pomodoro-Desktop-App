// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn set_title(app_handle: tauri::AppHandle, title: String) {
    if let Err(e) = app_handle.tray_handle().set_title(&title) {
        eprintln!("error updating timer: {}", e);
    }
}

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let window = event.window();
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![set_title])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
