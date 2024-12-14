use image::{DynamicImage, GenericImageView};
use crate::utils::base64_util::base64_encode;
use std::io::Cursor;

/// 将图像压缩返回
pub fn image_to_base64(image_path: &str, scale: f32) -> Result<String, Box<dyn std::error::Error>> {
    // 读取图像
    let img = image::open(image_path)?;

    // 获取图像的原始尺寸
    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);
    // 计算新的尺寸，按比例缩放
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    println!("Image dimensions: {}x{}", new_width, new_height);

    // 按比例缩放图像
    let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // 将图像保存为 PNG 格式的字节数组
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    resized_img.write_to(&mut cursor, image::ImageFormat::Jpeg)?;

    // 将字节数组编码为 Base64 字符串
    let base64_str = base64_encode(&bytes);

    Ok(base64_str)
}
