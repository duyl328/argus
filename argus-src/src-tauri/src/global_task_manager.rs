use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{mpsc, watch};
use tokio::task;

/// 后台任务自动管理
#[derive(Clone, Debug)]
pub struct BackgroundTaskAutoManager {
    /// 等待轮数
    wait_count: u16,
    /// 等待时长
    wait_duration: u16,
    /// 自动暂停【该值设置为 true 且 等待超过指定轮数自动暂停】
    auto_pause: bool,
    /// 自动开始【长时间等待后，任务进入，自动开始】
    auto_start: bool,
    /// 任务暂停时检查状态等待时长
    pause_check_duration: u16,
}

impl BackgroundTaskAutoManager {
    pub fn default() -> Self {
        Self {
            wait_count: 10,
            wait_duration: 10,
            auto_pause: false,
            auto_start: false,
            pause_check_duration: 3,
        }
    }
}

// 后台图像加载任务管理器
#[derive(Clone)]
pub(crate) struct BackgroundImageLoadingTaskManager {
    tx: mpsc::Sender<String>,
    /// 用于通知任务是否暂停
    pause_tx: watch::Sender<bool>,
    /// 自动任务
    auto_manager_rx: watch::Sender<BackgroundTaskAutoManager>,
}

impl BackgroundImageLoadingTaskManager {
    // 创建新的任务管理器
    pub(crate) fn new(
        tx: mpsc::Sender<String>,
        pause_tx: watch::Sender<bool>,
        auto_manager_rx: watch::Sender<BackgroundTaskAutoManager>,
    ) -> Self {
        Self {
            tx,
            pause_tx,
            auto_manager_rx,
        }
    }

    // 发送任务到后台队列
    pub(crate) async fn send_task(&self, task: String) {
        if let Err(_) = self.tx.send(task).await {
            eprintln!("Failed to send task");
        }
    }

    /// 更新后台任务
    pub async fn update_auto_manager(&self, auto_manager: BackgroundTaskAutoManager) {
        let _ = self.auto_manager_rx.send(auto_manager);
    }

    // 暂停后台任务
    pub(crate) async fn pause(&self) {
        let _ = self.pause_tx.send(true); // 向后台任务发送暂停信号
    }

    // 恢复后台任务
    pub(crate) async fn resume(&self) {
        let _ = self.pause_tx.send(false); // 向后台任务发送恢复信号
    }
}

// 启动后台任务，持续处理任务
pub async fn start_image_loading_background_task(
    mut rx: mpsc::Receiver<String>,
    mut pause_rx: watch::Receiver<bool>,
    auto_manager_rx: watch::Receiver<BackgroundTaskAutoManager>,
) {
    let mut paused = false;

    let mut wait_counter = 0; // 轮次计数器

    loop {
        tokio::select! {
            Some(task) = rx.recv() => {
                // 每次任务处理前，检查 auto_manager_rx 是否有更新
                let auto_manager = auto_manager_rx.borrow().clone();
                // 如果任务暂停，且不允许自动开始，等待
                if paused  {
                    // 如果自动开始，重置暂停状态
                        if  !auto_manager.auto_start{
                        paused = false;
                    }
                    // 如果不允许自动开始，进入睡眠，等待任务触发
                    println!("Waiting for auto_start flag to be true...");
                    tokio::time::sleep(Duration::from_secs(auto_manager.pause_check_duration.into())).await;
                    continue;
                }
                println!("Processing task: {}", task);
                tokio::time::sleep(Duration::from_secs(1)).await; // 模拟任务处理
                println!("Completed task: {}", task);
                // 清空计数
                wait_counter = 0;
                // 任务完成后，根据 auto_manager_rx 的状态来决定是否暂停
                if auto_manager.auto_pause {
                    println!("Automatically pausing after task completion...");
                    paused = true;
                }
            },

            // 后退避让算法，基于 auto_manager 中的 wait_duration 来调整等待时长
            _ = tokio::time::sleep(Duration::from_secs(auto_manager_rx.borrow().wait_duration.into())) => {
                // 如果超过了指定轮数，检查是否要暂停任务
                let auto_manager = auto_manager_rx.borrow().clone();

                if wait_counter >= auto_manager.wait_count {
                    if auto_manager.auto_pause {
                        println!("No tasks received for {} rounds, auto pausing...", wait_counter);
                        paused = true; // 达到等待轮数后自动暂停
                    }
                } else {
                    wait_counter += 1; // 递增等待轮数
                    println!("No tasks, waiting for new tasks... (round: {})", wait_counter);
                }
            },
            // 监听 pause_rx 的变化，如果接收到暂停命令，更新 paused 状态
            _ = pause_rx.changed() => {
                let is_paused = *pause_rx.borrow();
                paused = is_paused;
                if paused {
                    println!("Task paused due to pause_rx update.");
                } else {
                    println!("Task resumed due to pause_rx update.");
                }
            }
        }
    }
}
