use std::sync::Arc;
use crate::models::photo::Photo;
use crate::utils::img_util::ImageOperate;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, Mutex};
use tauri::async_runtime::Sender;
use crate::storage::connection::establish_connection;
use crate::storage::photo_table::insert_photo;
use crate::utils::task_util;

/// 数据库状态管理
pub static PHOTO_LOAD_RECEIVER: Lazy<Arc<Sender<ImageOperate>>> =
    Lazy::new(|| {
        // 使用 tokio 的 mpsc 通道
        let (photo_handler_tx, photo_handler_rx) = mpsc::channel::<ImageOperate>(100);

        let f = |io: ImageOperate| {
            let mut conn = establish_connection();
            insert_photo(&mut conn, io);
        };

        task_h(photo_handler_rx, f);

        Arc::new(photo_handler_tx)
    });

pub async fn task_h<T, F>(mut rx: mpsc::Receiver<T>, f: F)
where
    F: Fn(T),
{
    while let Some(task) = rx.recv().await {
        // 处理每个任务，单线程串行执行
        f(task)
    }
}
