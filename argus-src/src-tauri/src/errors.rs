use std::fmt;
use tauri::ipc::InvokeError;
use tauri::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // region common
    #[error("Default error {0}")]
    Error(#[from] diesel::result::Error),

    // endregion

    // region 文件操作
    #[error("IO error: {0}")]
    IoError(#[from] Error),

    #[error("文件路径不存在: {0}")]
    InvalidPath(String),

    #[error("指定文件不存在: {0}")]
    InvalidFile(String),

    #[error("文件保存失败: {0}")]
    FileSaveError(String),

    // endregion

    // region 数据库操作
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("数据库错误: {0}")]
    SqlError(#[from] SqlError),
    // endregion

    // region 系统配置
    #[error("系统配置 {0} 读取失败")]
    SysConfigReadError(String),
    // endregion
}

pub enum AError {
    /// 配置文件读取失败
    ThumbnailCacheConfigurationReadFailed,
    /// 原图读取失败
    OriginalImageReadFailed,
    /// Hash 转换失败
    HashConversionFailed,
    /// 文件保存失败
    FileSaveFailed,
    /// 缩略图生成失败
    ThumbnailGenerationFailed,

    /// 指定文件不存在
    SpecifiedFileDoesNotExist,
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
impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}

#[derive(Error, Debug)]
pub enum JsonError {
    /// 序列化失败
    #[error("序列化失败!")]
    SerializationFailed(),

    #[error("Default error {0}")]
    Error(#[from] serde_json::error::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum SqlError {
    #[error("Default error {0}")]
    Error(#[from] diesel::result::Error),

    #[error("数据插入失败 {0}")]
    InsertError(String),

    #[error("表: [{0}] 有效数据过多, 请检查数据库或删除部分数据!")]
    TooMuchValidData(String),

    #[error("未查询到指定数据!")]
    NotFound(),
}
