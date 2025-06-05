use axum::{middleware, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use crate::net_connection::ws_service::WebSocketState;

/// HTTP API 路由定义
pub fn create_http_routes() -> Router<WebSocketState> {
    Router::new()
        // 健康检查
        .route("/health", get(health_check))

        // 认证相关
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))

        // 照片相关API
        .route("/photos", get(list_photos))
        .route("/photos", post(upload_photo))
        .route("/photos/:id", get(get_photo))
        .route("/photos/:id", put(update_photo))
        .route("/photos/:id", delete(delete_photo))
        .route("/photos/:id/download", get(download_photo))

        // 任务相关API
        .route("/tasks/:id", get(get_task_status))
        .route("/tasks/:id/cancel", post(cancel_task))

        // 应用认证中间件【为整个 router 添加】
        // .layer(middleware::from_fn_with_state(AppState::default(), auth_middleware))
        // 不携带状态的中间件
        .layer(middleware::from_fn(auth_middleware1))
        // 只为 admin 路由添加
        // .route("/admin", get(login).route_layer(auth_layer))

}

// 认证中间件
async fn auth_middleware(
    State(state): State<WebSocketState>,
    mut request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Response {
    // 提取认证token
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    return next.run(request).await;
    
    // if let Some(token) = token {
    //     match state.services.auth_service.authenticate(token).await {
    //         Ok(Some(user_id)) => {
    //             // 将用户信息注入请求
    //             request.extensions_mut().insert(user_id);
    //         }
    //         _ => {
    //             return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    //         }
    //     }
    // }
    // 
    // next.run(request).await
}
// 认证中间件
async fn auth_middleware1(
    mut request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Response {
    // 提取认证token
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    next.run(request).await
}

// HTTP 处理器实现
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

async fn login() -> impl IntoResponse {
    // TODO: 实现登录逻辑
    Json(serde_json::json!({
        "token": "sample_token",
        "expires_in": 3600
    }))
}

async fn logout() -> impl IntoResponse {
    // TODO: 实现登出逻辑
    StatusCode::OK
}

async fn list_photos() -> impl IntoResponse {
    // TODO: 实现照片列表逻辑
    Json(serde_json::json!({
        "photos": [],
        "total": 0
    }))
}

async fn upload_photo(State(state): State<WebSocketState>) -> impl IntoResponse {
    // TODO: 实现照片上传逻辑
    // let task_id = state.services.task_service.create_task("upload".to_string(), "photo_upload".to_string()).await;

    Json(serde_json::json!({
        "task_id": "id",
        "message": "Upload started"
    }))
}

async fn get_photo(Path(id): Path<String>, State(state): State<WebSocketState>) -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

async fn update_photo(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: 实现照片信息更新逻辑
    StatusCode::OK
}

async fn delete_photo(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: 实现照片删除逻辑
    StatusCode::OK
}

async fn download_photo(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: 实现照片下载逻辑
    StatusCode::NOT_IMPLEMENTED
}

async fn get_task_status(Path(id): Path<String>, State(state): State<WebSocketState>) -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

async fn cancel_task(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: 实现任务取消逻辑
    StatusCode::OK
}
