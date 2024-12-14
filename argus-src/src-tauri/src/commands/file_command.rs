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
use serde::{Deserialize, Serialize};
use crate::utils::img_util::image_to_base64;
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
pub async fn get_dir_all_subfolders_first_img(path: String) -> Vec<FolderImage> {
    // 使用 spawn_blocking 将同步函数包装成异步
    let vec = get_all_subfolders(&path);
    let mut result: Vec<FolderImage> = Vec::new();
    // 使用并发处理文件夹
    for x in &vec {
        let display = x.display().to_string();

        // 对于每个文件夹，使用 spawn_blocking 获取文件夹中的图像路径
        let vec1 = get_all_dir_img(&display, Some(1)); // 获取文件夹中的图像路径
        if !vec1.is_empty() {
            let input = &*vec1[0];
            println!("123123123v {:?}", input);

            let ans = base64_encode(read_binary_file(input).expect("Base64 encode failed"));
            result.push(FolderImage {
                folder_path: x.display().to_string(),
                image_path_as_base64: ans,
            });
        }
    }

    result
}
// pub async fn get_dir_all_subfolders_first_img(path: String) -> Vec<FolderImage> {
//     // 使用 spawn_blocking 将同步函数包装成异步
//     let vec = task::spawn_blocking(move || get_all_subfolders(&path))
//         .await
//         .expect("Failed to get subfolders");
//
//     let mut result: Vec<FolderImage> = Vec::new();
//
//     // 使用并发处理文件夹
//     let mut tasks = vec![];
//
//     for x in &vec {
//         let display = x.display().to_string();
//
//         // 对于每个文件夹，使用 spawn_blocking 获取文件夹中的图像路径
//         // let task = task::spawn_blocking(move || get_all_dir_img(&display, Some(1)));
//         let vec1 = get_all_dir_img(&display, Some(1)); // 获取文件夹中的图像路径
//         if !vec1.is_empty() {
//
//             let ans = task::spawn_blocking(move || image_to_base64(&*vec1[0].clone(), 0.4).expect("报错了"));
//             // let base64 = image_to_base64(&*vec1[0].clone(), 0.4).expect("报错了");
//             tasks.push(ans);
//         }
//     }
//
//     // 等待所有任务完成并处理结果
//     let dir_imgs = futures::future::join_all(tasks).await;
//
//     for (folder, vec1) in vec.into_iter().zip(dir_imgs.into_iter()) {
//         result.push(FolderImage {
//             folder_path: folder.display().to_string(),
//             image_path_as_base64: vec1.expect("报错了"),
//         });
//     }
//
//     result
// }

#[derive(Serialize, Deserialize, Debug)]  // 需要加上这些
pub struct FolderImage {
    pub folder_path: String,
    pub image_path_as_base64: String,
}
