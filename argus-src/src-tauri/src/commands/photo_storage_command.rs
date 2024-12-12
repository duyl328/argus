use tauri::ipc::InvokeBody::Json;
use tauri::ipc::InvokeError;
use crate::errors::{AppError, SqlError};
use crate::models::photo_storage::PhotoStorage;
use crate::services;
use crate::utils::json_util::JsonUtil;

#[tauri::command]
pub fn get_photo_storage() -> String {
    let res = services::basic_service::get_photo_storages().expect("数据获取失败!");
    let result = JsonUtil::stringify(&res).expect("对象序列化失败!");
    result
}

#[tauri::command]
pub fn update_photo_storage(img_path: PhotoStorage) {
    services::basic_service::update_basic_setting_img_path(img_path).expect("基础设置图像地址更新失败!");
}
#[tauri::command]
pub fn add_photo_storage(imgath: PhotoStorage) -> Result<String, AppError> {
    log::info!("ans ssqweqweqwe");
    let img = JsonUtil::stringify(&imgath).expect("");
    log::info!("ans ss{}",img);
    let result = services::basic_service::add_basic_setting_img_path(imgath);
    if result.is_err() {
        Err(AppError::SqlError(result.unwrap_err()))
    } else {
        Ok("插入成功.".to_string())
    }
}
