use crate::commands::file_command::FolderImage;
use crate::utils::file_util::{get_all_dir_img, get_all_subfolders};

/// 获取需要展示的图片及信息
#[tauri::command]
pub fn get_need_display_image_info(path: String) -> Vec<FolderImage> {
    // 获取指定路径的图像并返回
    let vec = get_all_subfolders(&path);
    let mut result = vec![];
    for x in &vec {
        let display = x.display().to_string();

        // 只获取并展示第一张图片
        let vec1 = get_all_dir_img(&display, Some(1));
        result.push(FolderImage {
            folder_path: display,
            image_path_as_base64: vec1[0].clone(),
        });
    }
    result
}
