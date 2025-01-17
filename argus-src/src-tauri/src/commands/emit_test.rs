use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn emit_send_test(app: AppHandle, param: String) -> String {
    app.emit("download-started", format!("你好, {}! 来自后端!", param))
        .unwrap();
    "12389128391".to_string()
}
