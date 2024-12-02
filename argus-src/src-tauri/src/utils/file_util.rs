use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

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
    fs::copy(src_path, dest_path).map(|_| ()).map_err(|e| format!("拷贝文件失败: {}", e))
}

/// 移动文件（先拷贝后删除源文件）
pub fn move_file(src_path: &str, dest_path: &str) -> Result<(), String> {
    copy_file(src_path, dest_path)?;
    delete_file(src_path)
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
