use tauri::{AppHandle, Emitter};
use crate::constants::notify_message::DOWNLOAD_STARTED;

#[tauri::command]
pub async fn emit_send_test(app: AppHandle, param: String) -> String {
    app.emit(DOWNLOAD_STARTED, format!("你好, {}! 来自后端!", param))
        .unwrap();
    "12389128391".to_string()
}
