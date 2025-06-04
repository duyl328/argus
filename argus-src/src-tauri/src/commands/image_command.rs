use crate::commands::command::get_exif_info;
use crate::constants::app_config::{
    DEFAULT_THUMBNAIL_SIZE, IMAGE_COMPRESSION_RATIO, IMAGE_COMPRESSION_STORAGE_FORMAT,
};
use crate::errors::AError;
use crate::storage::photo::model;
use crate::storage::connection::establish_connection;
use crate::storage::schema::photo_table::img_path;
use crate::infra::config::SYS_CONFIG;
use crate::utils::exif_utils::exif_util;
use crate::utils::exif_utils::exif_util::ExifUtil;
use crate::utils::exif_utils::tag::Tags;
use crate::utils::file_util::{get_all_dir_img, get_all_subfolders};
use crate::utils::img_util::ImageOperate;
use crate::utils::json_util::JsonUtil;
use crate::utils::{file_util, img_util};
use anyhow::Result;
use tokio::task;
use crate::utils::task_util::{DbTask, DB_GLOBAL_TASK};

/// 压缩图片地址获取
///
/// 直接计算指定图片，如果不存在则压缩创建该图片后返回
/// 如果指定大小的图片未定义，则使用默认缩放大小
#[tauri::command]
pub fn get_compress_image_address(_path: String, size: u32) -> String {
    // 计算缩略图大小
    let mut image_size = DEFAULT_THUMBNAIL_SIZE;
    for x in IMAGE_COMPRESSION_RATIO {
        if x.size == size {
            image_size = x.size;
            break;
        }
    }

    let result = task::spawn_blocking(move || {});
    return "123123".parse().unwrap();
}

/// 生成保存缩略图
#[tauri::command]
pub async fn generate_save_thumbnail(image_path: Vec<String>, emit_command: String) -> Vec<String> {
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

    // let result1 = env::current_dir();
    // log::info!("跟文件路径: {}",result1.unwrap().display().to_string());

    // region 生成不同规格的缩略图
    let mut result = Vec::new();
    for x in need_to_compress_image {
        log::info!("正在压缩的文件路径 {}", x);
        let image_compression = ImageOperate::multi_level_image_compression(
            x,
            IMAGE_COMPRESSION_STORAGE_FORMAT,
            IMAGE_COMPRESSION_RATIO.to_vec(),
        );
        let vec1 = image_compression.await.expect("压缩文件路径获取失败");
        result.extend(vec1);
    }

    result
}

/// 获取指定图片的缩略图地址
#[tauri::command]
pub async fn get_image_thumbnail_path(image_path: String) -> Result<String, String> {
    // 判断文件是否存在
    if !file_util::file_exists(&image_path) {
        let string = AError::ThumbnailCacheConfigurationReadFailed
            .message()
            .to_string();
        log::error!("指定文件不存在 {} !", string);
        return Err(string);
    };
    // 获取 Hash
    let hash = file_util::sha256_async(&*image_path)
        .await
        .expect(AError::ThumbnailCacheConfigurationReadFailed.message());
    log::info!("file_util {}", hash);

    // 计算指定路径
    // 获取图片压缩比和图片压缩格式
    let root_dir = SYS_CONFIG.thumbnail_storage_path.clone().ok_or_else(|| {
        AError::ThumbnailCacheConfigurationReadFailed
            .message()
            .to_string()
    })?;
    // 获取文件名后缀
    let fmt = ImageOperate::get_suffix_name(IMAGE_COMPRESSION_STORAGE_FORMAT);
    let file_path =
        file_util::hash_to_file_path(&*hash, &*root_dir, &*fmt, DEFAULT_THUMBNAIL_SIZE);

    Ok(file_path.display().to_string())
}

/// 获取指定图片的缩略图【如果不存在，直接创建】
#[tauri::command]
pub async fn get_image_thumbnail(image_path: String) -> Result<String, String> {
    let string = ImageOperate::designate_level_image_compression(
        image_path,
        IMAGE_COMPRESSION_STORAGE_FORMAT,
        DEFAULT_THUMBNAIL_SIZE,
    )
    .await
    .map_err(|x| {
        let msg = x.to_string();
        log::error!("get_image_thumbnail error {}", x);
        if msg.trim().is_empty() {
            AError::ThumbnailGenerationFailed.message().to_string()
        } else {
            msg
        }
    })?;

    Ok(string)
}

/// 获取图像信息
#[tauri::command]
pub async fn get_image_info(image_path: String) -> Result<String, String> {
    // 判断文件是否存在
    if !file_util::file_exists(&image_path) {
        let string = "指定文件不存在".parse().unwrap();
        log::error!("指定文件不存在 {} !", string);
        return Err(string);
    };
    // 获取指定图像的 Hash
    let hash = file_util::sha256_async(&*image_path)
        .await
        .expect("获取文件 Hash 失败");

    let mut conn = establish_connection();
    // 查询数据库是否存在

    let option = crate::storage::photo::repository::find_photo_by_hash(&mut conn, hash).expect("查询出错");
    return match option {
        // 如果不存在，则创建
        None => {
            // 读取普通信息
            let image = ImageOperate::read_image(&image_path)
                .await
                .expect("图像信息读取失败！");
            // 读取 exif 信息
            let exif_tool = exif_util::ExifToolCmd;
            let exif_info = exif_tool
                .read_all_exif(&*image_path)
                .expect("图像信息读取失败！");
            let tag = Tags::new(true);
            let mt = tag.parse(&exif_info);
            let result = mt.pack_object().expect("数据打包失败！");

            let photo = crate::storage::photo::repository::merge_info(image.clone(), result.clone());
            let result1 = JsonUtil::stringify(&photo).expect("序列化失败");
            
            // 数据保存到数据库
            let db_task = DB_GLOBAL_TASK.clone();
            db_task
                .send(DbTask::PhotoFullInfoInsert(image,result))
                .expect("任务发送出错!");
            Ok(result1)
        }
        Some(res) => {
            let result = JsonUtil::stringify(&res).expect("序列化失败");
            Ok(result)
        }
    }
}
