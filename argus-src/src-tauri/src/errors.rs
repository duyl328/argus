use std::fmt;
use tauri::ipc::InvokeError;
use tauri::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AError {
    /// 配置文件读取失败
    #[error("Failed to read thumbnail cache configuration")]
    ThumbnailCacheConfigurationReadFailed,
    /// 原图读取失败
    #[error("Failed to read original image")]
    OriginalImageReadFailed,
    /// Hash 转换失败
    #[error("Failed to convert hash")]
    HashConversionFailed,
    /// 文件保存失败
    #[error("Failed to save file")]
    FileSaveFailed,
    /// 父路径读取失败
    #[error("Failed to read parent path")]
    ParentPathReadFailed,
    /// 缩略图生成失败
    #[error("Failed to generate thumbnail")]
    ThumbnailGenerationFailed,

    /// 指定文件不存在
    #[error("The specified file does not exist")]
    SpecifiedFileDoesNotExist,
    /// 指定配置文件不存在
    #[error("ConfigFileDoesNotExist")]
    ConfigFileDoesNotExist,
}

impl AError {
    pub fn code(&self) -> i32 {
        match self {
            AError::ThumbnailCacheConfigurationReadFailed => 1,
            AError::OriginalImageReadFailed => 2,
            AError::HashConversionFailed => 3,
            AError::FileSaveFailed => 3,
            AError::ThumbnailGenerationFailed => 3,
            AError::SpecifiedFileDoesNotExist => 3,
            AError::ParentPathReadFailed => 3,
            _ => -1
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            AError::ThumbnailCacheConfigurationReadFailed => "配置文件读取失败！",
            AError::OriginalImageReadFailed => "原图读取失败！",
            AError::HashConversionFailed => "Hash 转换失败！",
            AError::FileSaveFailed => "文件保存失败！",
            AError::ThumbnailGenerationFailed => "缩略图生成失败！",
            AError::SpecifiedFileDoesNotExist => "指定文件不存在！",
            AError::ParentPathReadFailed => "父路径读取失败！",
            _ => "默认报错返回"
        }
    }

    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            AError::ThumbnailCacheConfigurationReadFailed => Some("检查指定文件是否存在！"),
            _ => None,
        }
    }
}
// 实现 Into<InvokeError>
impl Into<InvokeError> for AError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}
