use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs;
use std::io::{self, Write};

/// 文本转换为 Base64
pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}

/// Base64 转换为文本
#[allow(dead_code)]
pub fn base64_to_text(encoded: &str) -> Result<String, String> {
    match STANDARD.decode(encoded) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => Ok(text),
            Err(_) => Err("Base64 解码后非 UTF-8 文本".to_string()),
        },
        Err(e) => Err(format!("解码 Base64 失败: {}", e)),
    }
}

#[allow(dead_code)]
/// Base64 转换为文件并写入
pub fn base64_to_file(encoded: &str, output_file_path: &str) -> Result<(), String> {
    match STANDARD.decode(encoded) {
        Ok(bytes) => match fs::File::create(output_file_path) {
            Ok(mut file) => {
                if file.write_all(&bytes).is_ok() {
                    Ok(())
                } else {
                    Err("写入文件失败".to_string())
                }
            }
            Err(e) => Err(format!("创建文件失败: {}", e)),
        },
        Err(e) => Err(format!("解码 Base64 失败: {}", e)),
    }
}
