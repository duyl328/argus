use crate::net_connection::ws_handler::handle_websocket;
use axum::extract::{Query, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::State, response::Response, Extension, Json};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{atomic::AtomicU64, Arc};
use tokio::sync::{broadcast, RwLock};
use tokio::sync::mpsc::UnboundedSender;

/// 统一消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub msg_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub user_id: Option<String>,
}
/// 共享应用状态
#[derive(Clone)]
pub struct WebSocketState {
    // WebSocket 连接管理
    pub connections: Arc<DashMap<String, ConnectionInfo>>,
    pub user_connections: Arc<DashMap<String, Vec<String>>>, // user_id -> connection_ids

    // 消息广播
    pub broadcast_tx: broadcast::Sender<Message>,
}
impl Default for WebSocketState {
    fn default() -> Self {
        Self {
            connections: Arc::new(Default::default()),
            user_connections: Arc::new(Default::default()),
            broadcast_tx: broadcast::channel(128).0,
        }
    }
}
/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub heartbeat_interval: u64,
    pub max_message_size: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_connections: 10000,
            heartbeat_interval: 30,
            max_message_size: 1024 * 1024, // 1MB
        }
    }
}

/// WebSocket 连接信息
#[derive(Debug)]
pub struct ConnectionInfo {
    pub id: String,
    pub user_id: Option<String>,
    pub connected_at: u64,
    pub last_ping: AtomicU64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageType {
    // 进度相关
    ProgressUpdate { task_id: String },
    ProgressComplete { task_id: String },

    // 系统消息
    Heartbeat,
    UserConnected,
    UserDisconnected,

    // 错误消息
    Error { code: u32, message: String },
}
type Clients = Arc<RwLock<HashMap<String, UnboundedSender<Message>>>>;

/// WebSocket 升级处理器
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

///
/// - ws 解析 HTTP 请求头和协议升级信息，封装为 WebSocketUpgrade
/// - params 从 URL 查询参数中提取，解析成 HashMap
/// - state 从应用的共享状态中拿到 WebSocketState
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<WebSocketState>,
) -> impl IntoResponse {
    if let Some(token) = params.get("token") {
        if validate_token(token) {
            let client_id = token.to_string();
            return ws.on_upgrade(move |socket| handle_websocket(socket, state));
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
fn validate_token(token: &str) -> bool {
    // 实际项目中请替换为 JWT 校验或 session 检查
    token.starts_with("valid-"); // 示例

    true
}
