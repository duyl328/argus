use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;


// 后台任务处理线程
async fn task_handler(mut rx: mpsc::Receiver<i32>, state: Arc<Mutex<i32>>) {
    while let Some(task_id) = rx.recv().await {
        // 处理每个任务，单线程串行执行
        write_to_database(task_id, state.clone()).await;
    }
    println!("No more tasks, waiting for new tasks...");
}
