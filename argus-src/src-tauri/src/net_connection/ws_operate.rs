use std::sync::Arc;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct ServiceLayer {
    pub auth_service: AuthService,
    pub photo_service: PhotoService,
    pub task_service: TaskService,
}

/// 认证服务接口
pub struct AuthService;

impl AuthService {
    pub async fn authenticate(&self, token: &str) -> Result<Option<String>, anyhow::Error> {
        // 这里只是占位符，返回用户ID或None
        if token.is_empty() {
            Ok(None)
        } else {
            Ok(Some(format!("user_{}", token)))
        }
    }

    pub async fn authorize(&self, user_id: &str, resource: &str, action: &str) -> Result<bool, anyhow::Error> {
        Ok(true) // 暂时允许所有操作
    }
}

/// 照片服务接口
pub struct PhotoService;

impl PhotoService {
    pub async fn get_photo_info(&self, photo_id: &str) -> Result<serde_json::Value, anyhow::Error> {
        Ok(serde_json::json!({
            "id": photo_id,
            "name": "sample.jpg",
            "size": 1024000,
            "created_at": chrono::Utc::now().timestamp()
        }))
    }

    pub async fn upload_photo(&self, user_id: &str, data: Vec<u8>) -> Result<String, anyhow::Error> {
        let photo_id = Uuid::new_v4().to_string();
        Ok(photo_id)
    }

    pub async fn delete_photo(&self, user_id: &str, photo_id: &str) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub async fn update_photo_info(&self, user_id: &str, photo_id: &str, info: serde_json::Value) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

/// 任务服务接口
pub struct TaskService {
    pub tasks: Arc<DashMap<String, TaskProgress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub id: String,
    pub user_id: String,
    pub progress: f32, // 0.0 - 1.0
    pub status: TaskStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
        }
    }

    pub async fn create_task(&self, user_id: String, task_type: String) -> String {
        let task_id = Uuid::new_v4().to_string();
        let task = TaskProgress {
            id: task_id.clone(),
            user_id,
            progress: 0.0,
            status: TaskStatus::Pending,
            message: Some(format!("Task {} created", task_type)),
        };
        self.tasks.insert(task_id.clone(), task);
        task_id
    }

    pub async fn update_progress(&self, task_id: &str, progress: f32, message: Option<String>) -> Result<(), anyhow::Error> {
        if let Some(mut task) = self.tasks.get_mut(task_id) {
            task.progress = progress.clamp(0.0, 1.0);
            task.status = if progress >= 1.0 { TaskStatus::Completed } else { TaskStatus::Running };
            task.message = message;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found"))
        }
    }

    pub async fn get_task(&self, task_id: &str) -> Option<TaskProgress> {
        self.tasks.get(task_id).map(|task| task.clone())
    }
}