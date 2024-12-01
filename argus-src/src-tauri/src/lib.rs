mod commands;
mod config;
mod constant;

use std::panic;
use tauri::{webview::{PageLoadEvent, WebviewWindowBuilder}, App, AppHandle, Emitter, Listener, Manager, RunEvent, WebviewUrl, WebviewWindow};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::command::greet,
            commands::file_command::get_image_absolute_path,
            commands::file_command::check_directory_access,
        ]).setup(|app| {


        Ok(())

    })
        .run(tauri::generate_context!())
        .expect("argus 启动失败!");
}
