use thiserror::Error;
use std::fmt;
use tauri::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] Error),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("默认错误!")]
    Error()
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
pub enum SqlError{
    #[error("Default error {0}")]
    Error(#[from] diesel::result::Error),

    #[error("数据插入失败 {0}")]
    InsertError(String),

    #[error("表: [{0}] 有效数据过多, 请检查数据库或删除部分数据!")]
    TooMuchValidData(String),

    #[error("未查询到指定数据!")]
    NotFound()
}

