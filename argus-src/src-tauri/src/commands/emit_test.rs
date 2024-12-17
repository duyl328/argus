use tauri::{AppHandle, Emitter};


#[tauri::command]
pub fn emit_send_test(app: AppHandle, param: String) {
    app.emit("download-started", format!("你好, {}! 来自后端!", param),
    ).unwrap();
}
