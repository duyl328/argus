use crate::global_task_manager::BackgroundImageLoadingTaskManager;
use anyhow::Result;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn add_task(
    global_task_manager: State<'_, Mutex<BackgroundImageLoadingTaskManager>>,
    task: String,
) -> Result<String, String> {
    println!("add_task: {}", task);
    global_task_manager.lock().await.send_task(task).await;
    // let manager = global_task_manager.lock();
    // manager.await.send_task(task).await;
    Ok(String::from("完成"))
}

#[tauri::command]
pub async fn pause_task(
    global_task_manager: State<'_, Arc<Mutex<BackgroundImageLoadingTaskManager>>>,
) -> Result<String, String> {
    println!("pause_task");
    let manager = global_task_manager.lock();
    manager.await.pause().await;
    Ok(String::from("完成"))
}

#[tauri::command]
pub async fn resume_task(
    global_task_manager: State<'_, Arc<Mutex<BackgroundImageLoadingTaskManager>>>,
) -> Result<String, String> {
    println!("resume_task");
    let manager = global_task_manager.lock();
    manager.await.resume().await;
    Ok(String::from("完成"))
}

#[tauri::command]
pub fn emit_global_msg(app: AppHandle) -> String {
    println!("emit_global_msg");
    // 访问存储的窗口实例
    app.emit("global-error-msg-display", "你好,  来自后端!".to_string())
        .unwrap();
    String::from("111111111 测试内哦啊违法我欸")
}
