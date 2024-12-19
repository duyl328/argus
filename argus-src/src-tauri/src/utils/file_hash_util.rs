use crate::utils::file_util::file_size;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub struct FileHashUtils;

impl FileHashUtils {
    /// 计算文件内容的 SHA-256 哈希值
    pub fn sha256(file_path: &str) -> std::io::Result<String> {
        let content = fs::read(file_path)?; // 一次性读取文件内容
        Ok(format!("{:x}", Sha256::digest(&content)))
    }

    pub async fn sha256_async(file_path: &str) -> io::Result<String> {
        let metadata = tokio::fs::metadata(file_path).await?;
        let file_size = metadata.len();

        // 缓冲区大小根据文件大小动态选择
        let m10 = 10 * 1024 * 1024; // 10 MB
        let m100 = 100 * 1024 * 1024; // 100 MB
        let buffer_size = if file_size < m10 {
            16 * 1024 // 小于等于 10MB，用 16KB 缓冲
        } else if file_size < m100 {
            64 * 1024 // 10MB 到 100MB 用 64KB
        } else {
            256 * 1024 // 超过 100MB，用 256KB 缓冲
        };

        let mut file = File::open(file_path).await?; // 异步打开文件
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; buffer_size];

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // 文件读取完毕
            }
            hasher.update(&buffer[..bytes_read]); // 更新哈希计算
        }

        Ok(format!("{:x}", hasher.finalize())) // 返回最终哈希值
    }

    /// Hash 文件路径生成
    pub fn get_hash_dir(sha: &str) -> PathBuf {
        // 目录级别【】

        todo!()
    }
}

#[test]
fn FileHashUtilsTest() -> std::io::Result<()> {
    let file_hash =
        FileHashUtils::sha256("D:/argus/img/jpg/局部/3f160e3827ea5aa149f3301a56b4f0a5.jpg")?;
    println!("File Hash (SHA256): {}", file_hash);
    Ok(())
}

#[tokio::test]
async fn main() -> io::Result<()> {
    let file_hash =
        FileHashUtils::sha256_async("D:/argus/img/jpg/局部/3f160e3827ea5aa149f3301a56b4f0a5.jpg")
            .await?;
    println!("File Hash (SHA256): {}", file_hash);

    let file_hash =
        FileHashUtils::sha256("D:/argus/img/jpg/局部/3f160e3827ea5aa149f3301a56b4f0a5.jpg")?;
    println!("File Hash (SHA256): {}", file_hash);

    Ok(())
}
