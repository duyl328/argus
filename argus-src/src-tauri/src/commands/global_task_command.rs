use crate::global_task_manager::BackgroundImageLoadingTaskManager;
use anyhow::Result;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn add_task(
    global_task_manager: State<'_, Mutex<BackgroundImageLoadingTaskManager>>,
    task: String,
) -> anyhow::Result<String, String> {
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
    print!("pause_task");
    let manager = global_task_manager.lock();
    manager.await.pause().await;
    Ok(String::from("完成"))
}

#[tauri::command]
pub async fn resume_task(
    global_task_manager: State<'_, Arc<Mutex<BackgroundImageLoadingTaskManager>>>,
) -> Result<String, String> {
    print!("resume_task");
    let manager = global_task_manager.lock();
    manager.await.resume().await;
    Ok(String::from("完成"))
}

#[tauri::command]
pub async fn set_app_handle(
    app: AppHandle,
    state: State<'_, Arc<Mutex<Option<AppHandle>>>>,
) -> Result<String, String> {
    // let mut state = state.lock().await;
    // 
    // // 解引用 MutexGuard 并更新其中的 Option<AppHandle>
    // *state = Some(app);

    Ok(String::from("保存成功!"))
}
