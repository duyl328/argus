mod commands;
mod constant;
mod utils;
mod storage;
mod services;
mod models;

use tauri::{Emitter, Listener, Manager};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::command::greet,
            commands::file_command::get_image_absolute_path,
            commands::file_command::check_directory_access,
            commands::file_command::read_image_as_base64,
            commands::post_command::get_all_post,
            commands::post_command::insert_post,
        ])
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("argus 启动失败!");
}
