use std::sync::Arc;
use image::imageops::FilterType;
use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, Semaphore};
use tokio::task;
use tokio::task::JoinSet;
use crate::commands::file_command::FolderImage;
use crate::constant::{DEFAULT_THUMBNAIL_SIZE, IMAGE_COMPRESSION_RATIO};
use crate::utils::file_util::{get_all_dir_img, get_all_subfolders};
use crate::utils::img_util::ImageOperate;

/// 压缩图片地址获取
///
/// 直接计算指定图片，如果不存在则压缩创建该图片后返回
/// 如果指定大小的图片未定义，则使用默认缩放大小
#[tauri::command]
pub async fn get_compress_image_address(path: String,size:u32) -> String {

    // 计算缩略图大小
    let mut image_size = DEFAULT_THUMBNAIL_SIZE;
    for x in IMAGE_COMPRESSION_RATIO {
        if x.size == size {
            image_size = x.size;
            break;
        }
    }


    let result = task::spawn_blocking(move || {

    });
    return "".parse().unwrap();
}

/// 生成保存缩略图
#[tauri::command]
pub async fn generate_save_thumbnail(app:AppHandle,image_path: Vec<String>,emit_command:String) {
    // region 整理所有需要压缩的目录
    let mut need_2_compressed_dir = vec![];
    image_path.iter().for_each(|x| {
        let dir = get_all_subfolders(x);
        need_2_compressed_dir.extend(dir);
    });
    // endregion

    // region 获取目录下所有的图片文件
    let mut need_to_compress_image = vec![];
    for x in need_2_compressed_dir {
        let display = x.display().to_string();
        let dir_img = get_all_dir_img(&display, Some(-1));
        need_to_compress_image.extend(dir_img);
    }
    // endregion

    // region 根据设置大小开始压缩


    // endregion

    // 使用 spawn_blocking 将同步函数包装成异步
    // let vec = get_all_subfolders(&path);
    // // let mut result: Vec<FolderImage> = Vec::new();
    // let mut result: Arc<Mutex<Vec<FolderImage>>> = Arc::new(Mutex::new(Vec::new())); // 使用 Arc 和 Mutex
    //
    // let semaphore = Arc::new(Semaphore::new(4)); // 限制并发任务数为 4
    // let mut join_set = JoinSet::new();
    //
    // let mut rs: Vec<(String, String)> = Vec::new();
    //
    // // 使用并发处理文件夹
    // for x in &vec {
    //     let display = x.display().to_string();
    //
    //     // 对于每个文件夹，使用 spawn_blocking 获取文件夹中的图像路径
    //     let vec1 = get_all_dir_img(&display, Some(-1)); // 获取文件夹中的图像路径
    //     for x in vec1 {
    //         rs.push((display.clone(), x.clone()));
    //     }
    // }
    //
    // for (dir_path, path) in rs {
    //     let app_hl = app.clone();
    //     let permit = semaphore.clone().acquire_owned().await.unwrap();
    //     let result = result.clone(); // 克隆 Arc，传递到异步任务中
    //     join_set.spawn(async move {
    //         // 读取
    //         let image = ImageOperate::read_image(&path).await?;
    //         // 压缩
    //         let compressed = image.compression_with_size(width, height, FilterType::Triangle).await;
    //         // Base64 转换
    //         let b64 = ImageOperate::get_base64(compressed).await?;
    //
    //         let mut result = result.lock().await;
    //
    //         let image1 = FolderImage {
    //             folder_path: dir_path,
    //             image_path_as_base64: b64,
    //         };
    //         app_hl.emit("folder-view-image-show", image1).unwrap();
    //         // result.push(image1);
    //         Ok::<(), anyhow::Error>(())
    //     });
    // }
    // while let Some(res) = join_set.join_next().await {
    //     if let Err(e) = res {
    //         eprintln!("任务失败: {}", e);
    //     }
    // }
    //
    // // join_set.
    // let guard = result.lock().await;
    // guard.clone()

}


/// 获取图片的 Hash
#[tauri::command]
pub fn get_image_hash(){

}
