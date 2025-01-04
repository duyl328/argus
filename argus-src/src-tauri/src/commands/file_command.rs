use crate::utils::base64_util::base64_encode;
use crate::utils::file_util::{
    file_exists, get_all_dir_img, get_all_img, get_all_subfolders, read_binary_file,
};
use crate::utils::json_util::JsonUtil;
use base64::Engine;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

/// 返回图像绝对路径
#[tauri::command]
pub fn get_image_absolute_path() -> String {
    let path = String::from("D:/argus/img/局部/5e9324ca411fa3f6.jpg");
    path
}

/// 检测是否有指定路径的访问权限
#[tauri::command]
pub fn check_directory_access(directory: String) -> bool {
    file_exists(&directory)
}

#[tauri::command]
pub fn read_image_as_base64(directory: String) -> Result<String, String> {
    // 检查文件是否存在
    if !file_exists(&directory) {
        return Err("File does not exist.".to_string());
    }

    // 读取照片
    let img = read_binary_file(&directory);
    match img {
        Ok(img) => {
            let result = base64_encode(img);
            Ok(result)
        }
        Err(err) => return Err(err.to_string()),
    }
}

/// 获取所有目录
#[tauri::command]
pub fn get_all_sub_dir(path: String) -> String {
    let vec = get_all_subfolders(&*path);
    JsonUtil::stringify(&vec).expect("JSON 转换失败")
}

#[tauri::command]
pub fn get_all_imgs(path: String) -> String {
    let vec = get_all_img(&*path);
    JsonUtil::stringify(&vec).expect("JSON 转换失败")
}

/// 获取指定路径下所有子文件夹的第一张图片
#[tauri::command]
pub async fn get_dir_all_subfolders_first_img(app: AppHandle, path: String) -> Vec<FolderImage> {
    // 使用 spawn_blocking 将同步函数包装成异步
    let vec = get_all_subfolders(&path);
    let mut result: Vec<FolderImage> = Vec::new(); // 使用 Arc 和 Mutex

    // 使用并发处理文件夹
    for x in &vec {
        let display = x.display().to_string();

        let vec1 = get_all_dir_img(&display, Some(1)); // 获取文件夹中的图像路径
        if !vec1.is_empty() {
            let path1 = vec1[0].clone();
            result.push(FolderImage {
                source_file_path: path1.clone(),
                file_path: display,
            });
        }
    }
    result
}

#[derive(Serialize, Deserialize, Debug, Clone)] // 需要加上这些
pub struct FolderImage {
    /// 原图路径
    pub source_file_path: String,
    /// 文件路径
    pub file_path: String,
}
