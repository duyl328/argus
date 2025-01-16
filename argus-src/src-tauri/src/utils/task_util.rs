use crate::models::photo::Photo;
use crate::utils::img_util::ImageOperate;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tauri::async_runtime::Sender;
/// 数据库状态管理
pub static PHOTO_LOAD_RECEIVER: Lazy<Arc<Mutex<Option<Sender<ImageOperate>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn task_h<T, F>(mut rx: mpsc::Receiver<T>, f: F)
where
    F: Fn(T),
{
    while let Some(task) = rx.recv().await {
        // 处理每个任务，单线程串行执行
        f(task)
    }
}
