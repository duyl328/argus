use crate::constant;
use crate::errors::{AError, AppError};
use crate::structs::config::SYS_CONFIG;
use crate::structs::image_size::ImageSize;
use crate::utils::base64_util::base64_encode;
use crate::utils::file_hash_util::FileHashUtils;
use crate::utils::file_util::file_exists;
use crate::utils::system_state_util::get_memory_as_percentage;
use crate::utils::{file_util, image_format_util};
use anyhow::{anyhow, Context, Result};
use env_logger::Target;
use image::{imageops, DynamicImage, GenericImageView, ImageError, ImageFormat};
use image::{imageops::FilterType, io::Reader as ImageReader};
use log::{error, info, warn};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use image::io::Reader;
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tokio::task::JoinSet;

#[derive(Debug, Clone)]
pub struct ImageOperate {
    /// 文件路径
    img_path: String,
    /// 图片存储
    data: DynamicImage,
}

impl ImageOperate {
    /// 读取图像到内存
    pub async fn read_image(image_path: &str) -> Result<ImageOperate> {
        let start_resize = Instant::now();
        // 检测文件是否存在
        if !file_exists(image_path) {
            return Err(anyhow!(AError::SpecifiedFileDoesNotExist.message()));
        };
        // 猜测文件类型并打开
        let result = image::ImageReader::open(image_path)?.with_guessed_format()?.decode()?;

        let rs = ImageOperate {
            img_path: String::from(image_path),
            data: result,
        };
        println!(
            "图片：{}, 读取: {:?}, 内存占用:{}",
            image_path,
            start_resize.elapsed(),
            get_memory_as_percentage()
        );
        Ok(rs)
    }

    /// 将图像压缩返回
    pub async fn compression(&self, scale: f32) -> Result<DynamicImage> {
        // 获取图像的原始尺寸
        let (width, height) = self.data.dimensions();
        // 计算新的尺寸，按比例缩放
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        // 按比例缩放图像
        let start_resize = Instant::now();
        let result = self
            .data
            .resize_exact(new_width, new_height, FilterType::Triangle);
        println!(
            "图片：{}, 压缩: {:?}, 内存占用:{}",
            self.img_path,
            start_resize.elapsed(),
            get_memory_as_percentage()
        );

        return Ok(result);
    }

    /// 按照比例缩放图片
    pub async fn compression_with_size(
        &self,
        new_width: u32,
        new_height: u32,
        filter: imageops::FilterType,
    ) -> DynamicImage {
        self.data.resize(new_width, new_height, filter)
    }

    /// 按照指定的宽高进行压缩
    pub async fn compression_with_size_exact(
        &self,
        new_width: u32,
        new_height: u32,
        filter: imageops::FilterType,
    ) -> DynamicImage {
        self.data.resize_exact(new_width, new_height, filter)
    }

    /// 转换为 BASE64
    pub async fn get_base64(img: DynamicImage) -> Result<String> {
        let mut bytes = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        img.write_to(&mut cursor, ImageFormat::Jpeg)?;
        let base64_str = base64_encode(&bytes);
        Ok(base64_str)
    }

    /// 保存图像到磁盘
    pub async fn save_image(
        path: String,
        image: DynamicImage,
        image_format: ImageFormat,
    ) -> Result<()> {
        let output_path = PathBuf::from(path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        let start_time = Instant::now();
        task::spawn_blocking(move || image.save_with_format(output_path, image_format)).await??;

        println!("保存文件: {:?} 完成", start_time.elapsed());
        Ok(())
    }

    /// 多级别图片压缩
    /// - dir 图像地址
    /// - fmt 压缩格式
    /// - compression_level 压缩级别
    pub async fn multi_level_image_compression(
        dir: String,
        fmt: ImageFormat,
        compression_level: Vec<ImageSize>,
    ) -> Result<Vec<String>> {
        // 结果图片地址
        let result = Arc::new(Mutex::new(Vec::new()));

        // 获取根目录
        let root_dir = Arc::new(SYS_CONFIG.thumbnail_storage_path.clone().unwrap());
        // 获取文件名
        let file_name = Arc::new(image_format_util::get_suffix_name(fmt.clone()));

        // 读取图片
        // let img = ImageOperate::read_image(dir).await?;
        let img = Arc::new(ImageOperate::read_image(&dir.clone()).await?); // 使用 Arc 包装图像
        let mut join_set = JoinSet::new();
        for level in compression_level {
            log::info!("获取图片尺寸 {}", &level.size);
            let vec_clone = Arc::clone(&result);
            let image = img.clone();
            let root_dir_clone = Arc::clone(&root_dir); // 克隆 Arc 来传递给闭包
            let file_name_clone = Arc::clone(&file_name);
            let dir_clone = dir.clone();
            join_set.spawn(async move {
                // 获取 Hash
                let hash = FileHashUtils::sha256_async(&dir_clone)
                    .await
                    .expect("读取文件 Hash 失败!");
                log::info!("获取 Hash ：{}", hash);
                // 获取保存路径
                let save_path = FileHashUtils::hash_to_file_path(
                    hash.as_str(),
                    &root_dir_clone,
                    &file_name_clone,
                    level.size,
                )
                .display()
                .to_string();
                log::info!("save_path {}", &save_path);
                // 保存结果数据
                let mut vec = vec_clone.lock().await;
                // 检测缩略图文件是否存在
                let exists = file_exists(&save_path);
                if !exists {
                    // 压缩
                    let x1 =
                        image.compression_with_size(level.size, level.size, FilterType::Triangle);
                    let image1 = x1.await;
                    // 保存
                    ImageOperate::save_image(save_path.clone(), image1, fmt)
                        .await
                        .expect("文件报错失败! ");
                }
                vec.push(save_path)
            });
        }

        while let Some(res) = join_set.join_next().await {
            if let Err(e) = res {
                eprintln!("任务失败: {}", e);
            }
        }
        let vec = {
            let vec = result.lock().await; // 再次获取锁
            vec.clone() // 克隆 Vec 以得到一个新的 Vec
        };
        Ok(vec)
    }

    /// 生成指定级别的压缩图
    pub async fn designate_level_image_compression(
        dir: String,
        fmt: ImageFormat,
        compression_level: u32,
    ) -> Result<String> {
        // 获取根目录
        let root_dir = SYS_CONFIG
            .thumbnail_storage_path
            .clone()
            .ok_or_else(|| anyhow!(AError::ThumbnailCacheConfigurationReadFailed.message()))?;
        // 获取文件名
        let file_name = image_format_util::get_suffix_name(fmt.clone());

        // 获取 Hash
        let hash = FileHashUtils::sha256_async(&dir)
            .await
            .map_err(|e| anyhow!(AError::HashConversionFailed.message()))?;
        log::info!("获取 Hash ：{}", hash);
        // 获取保存路径
        let save_path = FileHashUtils::hash_to_file_path(
            hash.as_str(),
            &root_dir,
            &file_name,
            compression_level,
        )
        .display()
        .to_string();
        log::info!("save_path {}", &save_path);

        // 检测缩略图文件是否存在
        let exists = file_exists(&save_path);
        if !exists {
            // 读取图片
            // let img = ImageOperate::read_image(dir).await?;
            let img = ImageOperate::read_image(&dir.clone()).await.map_err(|e| {
                let err = e.to_string();
                return if err.is_empty() {
                    anyhow!(format!("file: {} ,压缩失败: {}", dir, AError::OriginalImageReadFailed.message()))
                } else {
                    anyhow!(format!("file: {} ,压缩失败: {}", dir, err))
                };
            })?;
            // 压缩
            let x1 = img.compression_with_size(
                compression_level,
                compression_level,
                FilterType::Triangle,
            );
            let image1 = x1.await;
            // 保存
            ImageOperate::save_image(save_path.clone(), image1, fmt)
                .await
                .map_err(|e| anyhow!(AError::FileSaveFailed.message()))?;
        }

        Ok(save_path)
    }
}

/// 图像压缩测试
#[tokio::test]
async fn test_async_function() {
    let str = "D:/argus/img/img1.jpg";

    let image_paths = vec![
        "D:/argus/img/1.jpg",
        "D:/argus/img/2.jpg",
        "D:/argus/img/3.jpg",
        "D:/argus/img/4.jpg",
        "D:/argus/img/5.jpg",
        "D:/argus/img/6.jpg",
        "D:/argus/img/7.jpg",
        "D:/argus/img/8.jpg",
        "D:/argus/img/9.jpg",
        "D:/argus/img/10.jpg",
    ];

    let mut join_set = JoinSet::new();
    for path in image_paths {
        join_set.spawn(async move {
            // 读取
            let image = ImageOperate::read_image(&path).await?;
            // 压缩
            let compressed = image.compression(0.3).await?;
            // 保存
            ImageOperate::save_image(image.img_path, compressed, ImageFormat::Jpeg).await?;
            Ok::<(), anyhow::Error>(())
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            eprintln!("任务失败: {}", e);
        }
    }

    return;

    // 创建一个 mpsc 通道，缓冲区大小为 4

    // 定义两个 mpsc 通道
    let (tx_read_to_compress, mut rx_read_to_compress) = mpsc::channel::<ImageOperate>(4);
    let (tx_compress_to_save, mut rx_compress_to_save) = mpsc::channel::<(String, DynamicImage)>(4);

    // 生产者：读取任务
    let producer = tokio::spawn({
        async move {
            for path in image_paths {
                match ImageOperate::read_image(path).await {
                    Ok(image) => {
                        tx_read_to_compress.send(image).await.unwrap();
                    }
                    Err(e) => eprintln!("读取错误: {}", e),
                }
            }
            drop(tx_read_to_compress); // 关闭发送端，通知压缩线程
        }
    });

    // 中间阶段：压缩任务
    let compressor = tokio::spawn({
        async move {
            while let Some(image) = rx_read_to_compress.recv().await {
                match image.compression(0.3).await {
                    Ok(compressed_image) => {
                        tx_compress_to_save
                            .send((image.img_path, compressed_image))
                            .await
                            .unwrap();
                    }
                    Err(e) => eprintln!("压缩错误: {}", e),
                }
            }
            drop(tx_compress_to_save); // 关闭发送端，通知保存线程
        }
    });

    // 消费者：保存任务
    let saver = tokio::spawn(async move {
        while let Some((path, image)) = rx_compress_to_save.recv().await {
            if let Err(e) = ImageOperate::save_image(path, image, ImageFormat::Jpeg).await {
                eprintln!("保存错误: {}", e);
            }
        }
    });

    // 等待所有任务完成
    producer.await.expect("失败1");
    compressor.await.expect("失败2");
    saver.await.expect("失败3");

    println!("所有任务完成！");

    // let start_resize = Instant::now();
    // let image = ImageOperate::read_image(str).await.expect("文件读取出错");
    // let image1 = image.compression(0.3).await.expect("文件缩放出错");
    // image1.save_with_format("D:/argus/img/img1222.jpg", ImageFormat::Jpeg).expect("保存文件出错");
    // println!("总用时:{:?}", start_resize.elapsed());
}
