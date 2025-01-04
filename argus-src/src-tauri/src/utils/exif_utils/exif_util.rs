use crate::utils::exif_utils::tag::Tag;
use crate::utils::file_util;
use anyhow::{anyhow, Result};
use futures::io::ReadExact;
use futures::{AsyncReadExt, AsyncWriteExt};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    /// exiftool 路径
    static ref EXIF_CMD_PATH: RwLock<Option<String>> = RwLock::new(None);
    static ref INIT: AtomicBool = AtomicBool::new(false);
}

trait ExifUtil {
    /// 读取所有 exif 信息
    fn read_all_exif(&self, path: &str) -> Result<String>;
    fn write_exif(&self, exif_data: Vec<u8>);
}

struct ExifToolCmd;
struct ArgusExif;

impl ExifUtil for ExifToolCmd {
    fn read_all_exif(&self, path: &str) -> Result<String> {
        // 检测文件是否存在
        if !file_util::file_exists(path) {
            return Err(anyhow!("文件不存在"));
        }

        let exiftool_path = ExifToolCmd::get_exiftool_path();
        if !file_util::file_exists(exiftool_path.as_str()) {
            return Err(anyhow!("执行文件 exiftool 不存在! "));
        }

        let output = std::process::Command::new(exiftool_path.as_str())
            .arg(path)
            .output()
            .expect("failed to execute process");

        // 检查命令是否成功执行
        if output.status.success() {
            // 获取标准输出并转为字符串
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.into_owned())
        } else {
            // 获取标准错误并转为字符串
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!(stderr.to_string()))
        }
    }

    fn write_exif(&self, exif_data: Vec<u8>) {
        let mut child = std::process::Command::new("exiftool")
            .arg("-ExifIFD=")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(&exif_data)
            .expect("Failed to write to stdin");
    }
}

impl ExifToolCmd {
    /// 获取 exiftool 路径
    fn get_exiftool_path() -> Arc<String> {
        // 使用 AtomicBool 确保只初始化一次
        if !INIT.load(Ordering::Acquire) {
            let mut exif_cmd_path = EXIF_CMD_PATH.write().unwrap();
            if exif_cmd_path.is_none() {
                let path =
                    String::from("D:/argus/argus-src/src-tauri/resources/exiftool/exiftool.exe");
                *exif_cmd_path = Some(path);
            }
            INIT.store(true, Ordering::Release); // 标记初始化完成
        }

        // 返回 Arc<String>，避免复制数据
        let exif_cmd_path = EXIF_CMD_PATH.read().unwrap();
        Arc::new(exif_cmd_path.as_ref().unwrap().clone()) // 克隆数据并用 Arc 包装，避免重复复制
    }
}

mod test {
    use super::*;

    #[test]
    fn test_exif_tool() {
        let exif_tool = ExifToolCmd;
        let exif_data =
            exif_tool.read_all_exif("./resources/image/image-1-1.JPG").unwrap();
        let mut tag = Tag::new();
        let mt = tag.parse(&exif_data);
        let option = mt
            .entry_map
            .get(crate::utils::exif_utils::tag::ExifToolDesc::MAKE.exif_tool_desc);
        println!("{:?}", option);
    }

}
