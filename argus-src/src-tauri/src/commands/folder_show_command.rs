use crate::utils::file_util::{get_all_dir_img, get_all_subfolders};

/// 获取需要展示的图片及信息
#[tauri::command]
pub fn get_need_display_image_info(path: String) {}
