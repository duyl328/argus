#[tauri::command]
pub fn get_all_post() {
    log::info!("get all post");
    crate::services::post_service::get_all_post()
}

#[tauri::command]
pub fn insert_post() {
    crate::services::post_service::insert_post()
}
