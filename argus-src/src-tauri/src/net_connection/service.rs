use crate::infra::config::SYS_CONFIG;
use crate::net_connection::http_service::create_http_routes;
use crate::net_connection::ws_service::{websocket_handler, ws_handler, AppConfig, WebSocketState};
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::fmt::format;

/// 启动服务器
pub async fn start_server() -> Result<(), anyhow::Error> {
    let state = WebSocketState::default();
    let app = create_app(state);

    let host = SYS_CONFIG.host.clone().unwrap();
    let port = SYS_CONFIG.port.clone().unwrap();

    // 获取一个空闲端口
    // let std_listener = std::net::TcpListener::bind(format!("127.0.0.1:{port}").as_str()).unwrap();
    // 
    // let socket_addr = std_listener.local_addr().unwrap();
    // let port = socket_addr.port();
    // println!("使用的端口是：{:?}", socket_addr);

    // netstat -ano | findstr :51310

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    let socket_addr = listener.local_addr().unwrap();
    let port = socket_addr.port();
    println!("使用的端口是：{:?}", socket_addr);


    // tracing::info!("Server starting on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// 创建完整的应用路由
pub fn create_app(state: WebSocketState) -> Router {
    Router::new()
        // WebSocket 路由
        .route("/ws", get(ws_handler))
        // HTTP API 路由
        .merge(create_http_routes())
        // 静态文件服务（可选）
        // .nest_service("/static", tower_http::services::ServeDir::new("static"))
        // 应用状态
        .with_state(state)
        // 中间件
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
}
/// 优雅关闭信号处理
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting shutdown");
}
