#[tauri::command]
pub fn log_logs() {
    log::info!("正常信息");
    log::warn!("正常信息");
    log::error!("正常信息");
    log::debug!("正常信息");
}
