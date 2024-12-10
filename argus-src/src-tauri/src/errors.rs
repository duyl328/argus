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

