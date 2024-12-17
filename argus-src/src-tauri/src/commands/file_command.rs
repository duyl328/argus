use crate::utils::base64_util::base64_encode;
use crate::utils::file_util::{file_exists, get_all_dir_img, get_all_img, get_all_subfolders, read_binary_file};
use base64::encode;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::slice::RChunksExactMut;
use std::sync::Arc;
use image::imageops::FilterType;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Semaphore};
use tokio::task::JoinSet;
use crate::utils::img_util::ImageOperate;
use crate::utils::json_util::JsonUtil;

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
pub async fn get_dir_all_subfolders_first_img(path: String, width: u32, height: u32) -> Vec<FolderImage> {
    // 使用 spawn_blocking 将同步函数包装成异步
    let vec = get_all_subfolders(&path);
    // let mut result: Vec<FolderImage> = Vec::new();
    let mut result: Arc<Mutex<Vec<FolderImage>>> = Arc::new(Mutex::new(Vec::new())); // 使用 Arc 和 Mutex

    let semaphore = Arc::new(Semaphore::new(4)); // 限制并发任务数为 4
    let mut join_set = JoinSet::new();

    let mut rs: Vec<(String, String)> = Vec::new();

    // 使用并发处理文件夹
    for x in &vec {
        let display = x.display().to_string();

        // 对于每个文件夹，使用 spawn_blocking 获取文件夹中的图像路径
        let vec1 = get_all_dir_img(&display, Some(1)); // 获取文件夹中的图像路径
        if !vec1.is_empty() {
            // let input = &*vec1[0].clone();
            // let input = &*vec1[0].clone(); // 这里克隆元素，而不是借用
            let input = vec1[0].clone(); // 克隆元素，获取值的所有权
            rs.push((display, input));
        }
    }

    for (dir_path, path) in rs {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let result = result.clone(); // 克隆 Arc，传递到异步任务中
        join_set.spawn(async move {
            // 读取
            let image = ImageOperate::read_image(&path).await?;
            // 压缩
            let compressed = image.compression_with_size(width,height,FilterType::Triangle).await;
            // Base64 转换
            let b64 = ImageOperate::get_base64(compressed).await?;

            let mut result = result.lock().await;
            result.push(FolderImage {
                folder_path: dir_path,
                image_path_as_base64: b64,
            });
            Ok::<(), anyhow::Error>(())
        });
    }
    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            eprintln!("任务失败: {}", e);
        }
    }

    // join_set.
    let guard = result.lock().await;
    guard.clone()
}

#[derive(Serialize, Deserialize, Debug,Clone)]  // 需要加上这些
pub struct FolderImage {
    pub folder_path: String,
    pub image_path_as_base64: String,
}
