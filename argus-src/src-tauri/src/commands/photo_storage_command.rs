use crate::models::photo_storage::PhotoStorage;
use crate::services;
use crate::utils::json_util::JsonUtil;

#[tauri::command]
pub fn get_photo_storage() -> String {
    let res = services::photo_photo_service::get_photo_storages().expect("数据获取失败!");
    let result = JsonUtil::stringify(&res).expect("对象序列化失败!");
    result
}

#[tauri::command]
pub fn update_photo_storage(img_path: PhotoStorage) {
    services::photo_photo_service::update_basic_setting_img_path(img_path)
        .expect("基础设置图像地址更新失败!");
}

#[tauri::command]
pub fn delete_photo_storage(id: i32) {
    services::photo_photo_service::delete_img_path(id).expect("基础设置图像地址删除失败!");
}

#[tauri::command]
pub fn add_photo_storage(img2path: String, is_enable: bool) -> Result<String, String> {
    let result = services::photo_photo_service::add_img_path(img2path, is_enable);
    if result.is_err() {
        Err("数据库读取失败！".to_string())
    } else {
        Ok("插入成功.".to_string())
    }
}
