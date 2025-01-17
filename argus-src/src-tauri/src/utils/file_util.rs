use crate::errors::AError;
use anyhow::{anyhow, Result};
use glob::glob;
use sha2::digest::typenum::op;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 读取文本文件内容
pub fn read_text_file(file_path: &str) -> Result<String, String> {
    fs::read_to_string(file_path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 写入文本到文件
pub fn write_text_file(file_path: &str, content: &str) -> Result<(), String> {
    fs::write(file_path, content).map_err(|e| format!("写入文件失败: {}", e))
}

/// 读取二进制文件内容
pub fn read_binary_file(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 写入二进制数据到文件
pub fn write_binary_file(file_path: &str, data: &[u8]) -> Result<(), String> {
    File::create(file_path)
        .and_then(|mut file| file.write_all(data))
        .map_err(|e| format!("写入二进制文件失败: {}", e))
}

/// 检查文件是否存在
pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

/// 获取文件大小
pub fn file_size(file_path: &str) -> Result<u64, String> {
    fs::metadata(file_path)
        .map(|metadata| metadata.len())
        .map_err(|e| format!("获取文件大小失败: {}", e))
}

/// 创建目录
pub fn create_directory(dir_path: &str) -> Result<(), String> {
    fs::create_dir_all(dir_path).map_err(|e| format!("创建目录失败: {}", e))
}

/// 删除文件
pub fn delete_file(file_path: &str) -> Result<(), String> {
    fs::remove_file(file_path).map_err(|e| format!("删除文件失败: {}", e))
}

/// 删除目录及其内容
pub fn delete_directory(dir_path: &str) -> Result<(), String> {
    fs::remove_dir_all(dir_path).map_err(|e| format!("删除目录失败: {}", e))
}

/// 列出目录中的文件
pub fn list_directory(dir_path: &str) -> Result<Vec<PathBuf>, String> {
    fs::read_dir(dir_path)
        .map_err(|e| format!("读取目录失败: {}", e))?
        .map(|entry| entry.map(|e| e.path()).map_err(|e| format!("{}", e)))
        .collect()
}

/// 拷贝文件
pub fn copy_file(src_path: &str, dest_path: &str) -> Result<(), String> {
    fs::copy(src_path, dest_path)
        .map(|_| ())
        .map_err(|e| format!("拷贝文件失败: {}", e))
}

/// 移动文件（先拷贝后删除源文件）
pub fn move_file(src_path: &str, dest_path: &str) -> Result<(), String> {
    copy_file(src_path, dest_path)?;
    delete_file(src_path)
}

/// 获取所有指定文件夹的子目录
pub fn get_all_subfolders(path: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .min_depth(0) // 忽略起始目录本身
        .into_iter()
        .filter_map(|entry| entry.ok()) // 忽略无效条目
        .filter(|entry| entry.file_type().is_dir()) // 只保留文件夹
        .map(|entry| entry.path().to_path_buf()) // 转换为 PathBuf
        .collect()
}

/// 获取所有照片
pub fn get_all_img(path: &str) -> Vec<String> {
    let vec = get_all_subfolders(path);
    let mut res:Vec<String> = Vec::new();
    for x in vec {
        let display = x.display().to_string();
        get_all_dir_img(&*display, Some(-1));
        res.push(display);
    }
    res
}

/// 获取指定路径下所有图片
/// * `path` 指定路径
/// * `img_num` 获取多少张图片，如果是0直接返回，如果为负数则获取所有图片
pub fn get_all_dir_img(path: &str, img_num: Option<i32>) -> Vec<String> {
    let mut i = 0;
    // 默认张数
    let nums = img_num.unwrap_or(-1);
    if nums == 0 {
        return [].to_vec();
    }
    let valid_extensions = ["jpg", "png", "gif", "jpeg"]; // 图片文件扩展名
                                                          // 数据返回合集
    let mut all_img: Vec<String> = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if valid_extensions.contains(&extension.to_str().unwrap_or_default()) {
                            i += 1;
                            let x = i == nums;
                            all_img.push(String::from(path.to_str().unwrap()));
                            if x {
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory.");
    }
    all_img
}

/// 获取运行环境文件路径根目录
pub fn get_root_folder() -> Result<PathBuf> {
    // 获取基础路径，默认为当前 EXE 所在目录
    let buf = std::env::current_exe()?
        .parent()
        .ok_or(anyhow!(AError::ParentPathReadFailed))?
        .to_path_buf();
    Ok(buf)
}

/// 创建指定的文件夹
pub fn create_folder(base_dir: Option<&str>, relative_path: &str) -> Result<String, String> {
    // 获取基础路径，默认为当前 EXE 所在目录
    let base_path = if let Some(dir) = base_dir {
        PathBuf::from(dir)
    } else {
        get_root_folder().unwrap()
    };
    log::info!("创建指定的目录{}", base_path.display());

    // 拼接目标路径
    let target_path = base_path.join(relative_path);
    if target_path.exists() {
        log::info!("文件或目录已存在! ");
        return Ok(target_path.to_str().unwrap().to_string());
    }
    // 检查是否已经存在同名文件
    if target_path.is_file() {
        return Err(format!(
            "A file with the same name already exists: {}",
            target_path.display()
        ));
    }

    // 创建目录
    fs::create_dir_all(&target_path).map_err(|e| format!("Failed to create folder: {}", e))?;

    Ok(target_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*; // 引入当前模块的所有内容

    #[test]
    fn test_write_and_read_text_file() {
        let file_path = "test_file.txt";
        let content = "Hello, test!";

        // 测试写入
        assert!(write_text_file(file_path, content).is_ok());

        // 测试读取
        let read_content = read_text_file(file_path).expect("读取失败");
        assert_eq!(read_content, content);

        // 清理
        assert!(delete_file(file_path).is_ok());
    }

    #[test]
    fn test_file_exists() {
        let file_path = "test_existence.txt";
        write_text_file(file_path, "test").unwrap();

        // 文件应该存在
        assert!(file_exists(file_path));

        // 删除文件
        delete_file(file_path).unwrap();

        // 文件不再存在
        assert!(!file_exists(file_path));
    }

    #[test]
    fn test_file_size() {
        let file_path = "test_size.txt";
        let content = "123456";
        write_text_file(file_path, content).unwrap();

        // 文件大小应该匹配
        let size = file_size(file_path).unwrap();
        assert_eq!(size, content.len() as u64);

        // 清理
        delete_file(file_path).unwrap();
    }
}
