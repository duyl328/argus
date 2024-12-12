use crate::services;
use crate::utils::json_util::JsonUtil;

#[tauri::command]
pub fn get_basic_setting() -> String {
    let res = services::basic_service::get_basic_setting().expect("数据获取失败!");
    let result = JsonUtil::stringify(&res).expect("对象序列化失败!");
    result
}
pub fn update_basic_setting_img_path(imgs: String) {
    services::basic_service::update_basic_setting_img_path(imgs).expect("基础设置图像地址更新失败!");
}
