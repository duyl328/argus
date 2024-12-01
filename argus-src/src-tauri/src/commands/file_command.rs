use std::path::{Path, PathBuf};
use std::fs;
/// 返回图像绝对路径
#[tauri::command]
pub fn get_image_absolute_path() -> String {
    let path = String::from("D:/argus/img/局部/5e9324ca411fa3f6.jpg");
    path
}



#[tauri::command]
pub fn get_image_as_base64() -> String {
    let path = String::from("D:/argus/img/局部/5e9324ca411fa3f6.jpg");
    path
}





/// 检测是否有指定路径的访问权限
#[tauri::command]
pub fn check_directory_access(directory: String) -> Result<bool, String> {
    let path = Path::new(&directory);
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    } else {
        Err("Path is not a directory.".to_string())
    }
}
