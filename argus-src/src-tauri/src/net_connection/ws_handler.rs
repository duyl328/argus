use crate::net_connection::ws_service::{WebSocketState, ConnectionInfo, Message, MessageType};
use axum::extract::ws::WebSocket;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use futures_util::StreamExt;
use uuid::Uuid;

/// WebSocket 连接处理逻辑
pub async fn handle_websocket(socket: WebSocket, state: WebSocketState) {
    let connection_id = Uuid::new_v4().to_string();
    let connection_info = ConnectionInfo {
        id: connection_id.clone(),
        user_id: None, // 初始时未认证
        connected_at: chrono::Utc::now().timestamp() as u64,
        last_ping: AtomicU64::new(chrono::Utc::now().timestamp() as u64),
    };

    // 注册连接
    state
        .connections
        .insert(connection_id.clone(), connection_info);

    // 创建消息接收器
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    // 分离发送和接收
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    // 处理接收到的消息
    let recv_state = state.clone();
    let recv_connection_id = connection_id.clone();
    let recv_sender = sender.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if let Err(e) =
                    handle_websocket_message(msg, &recv_connection_id, &recv_state, &recv_sender)
                        .await
                {
                    tracing::error!("Error handling WebSocket message: {}", e);
                    break;
                }
            }
        }
    });

    // 处理广播消息
    let broadcast_sender = sender.clone();
    let broadcast_connection_id = connection_id.clone();
    let broadcast_state = state.clone();
    let broadcast_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            // 检查消息是否应该发送给当前连接
            if should_send_to_connection(&msg, &broadcast_connection_id, &broadcast_state).await {
                let json_msg = serde_json::to_string(&msg).unwrap_or_default();
                let mut sender = broadcast_sender.lock().await;
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = recv_task => {},
        _ = broadcast_task => {},
    }

    // 清理连接
    cleanup_connection(&connection_id, &state).await;
}

/// 处理单个 WebSocket 消息
async fn handle_websocket_message(
    msg: axum::extract::ws::Message,
    connection_id: &str,
    state: &WebSocketState,
    sender: &Arc<
        tokio::sync::Mutex<futures_util::stream::SplitSink<WebSocket, axum::extract::ws::Message>>,
    >,
) -> Result<(), anyhow::Error> {
    match msg {
        axum::extract::ws::Message::Binary(_) => {
            // 处理二进制消息（如图片数据）
            // TODO: 实现二进制消息处理
        }
        axum::extract::ws::Message::Ping(data) => {}
        axum::extract::ws::Message::Pong(_) => {
            // 更新心跳时间
            if let Some(conn) = state.connections.get(connection_id) {
                conn.last_ping
                    .store(chrono::Utc::now().timestamp() as u64, Ordering::Relaxed);
            }
        }
        axum::extract::ws::Message::Close(_) => {
            return Err(anyhow::anyhow!("Connection closed"));
        }
        _ => {}
    }
    Ok(())
}

/// 处理解析后的消息
async fn handle_parsed_message(
    msg: Message,
    connection_id: &str,
    state: &WebSocketState,
    sender: &Arc<
        tokio::sync::Mutex<futures_util::stream::SplitSink<WebSocket, axum::extract::ws::Message>>,
    >,
) -> Result<(), anyhow::Error> {
    match &msg.msg_type {
        MessageType::Heartbeat => {
            // 回应心跳
            let response = Message {
                id: Uuid::new_v4().to_string(),
                msg_type: MessageType::Heartbeat,
                payload: serde_json::json!({"status": "alive"}),
                timestamp: chrono::Utc::now().timestamp() as u64,
                user_id: msg.user_id,
            };
            send_message_to_connection(response, sender).await?;
        }
        _ => {
            // 其他消息类型的处理
            tracing::info!("Received message type: {:?}", msg.msg_type);
        }
    }
    Ok(())
}


/// 发送消息到特定连接
async fn send_message_to_connection(
    msg: Message,
    sender: &Arc<
        tokio::sync::Mutex<futures_util::stream::SplitSink<WebSocket, axum::extract::ws::Message>>,
    >,
) -> Result<(), anyhow::Error> {
    // let json_msg = serde_json::to_string(&msg)?;
    // let sender = sender.lock().await;
    // sender.send(axum::extract::ws::Message::Text(Utf8Bytes::from(json_msg))).await?;
    Ok(())
}

/// 判断消息是否应该发送给特定连接
async fn should_send_to_connection(msg: &Message, connection_id: &str, state: &WebSocketState) -> bool {
    // 如果消息指定了用户ID，只发送给该用户的连接
    if let Some(target_user_id) = &msg.user_id {
        if let Some(conn) = state.connections.get(connection_id) {
            return conn.user_id.as_ref() == Some(target_user_id);
        }
        return false;
    }

    // 广播消息发送给所有连接
    true
}

/// 清理连接资源
async fn cleanup_connection(connection_id: &str, state: &WebSocketState) {
    if let Some((_, conn)) = state.connections.remove(connection_id) {
        // 从用户连接映射中移除
        if let Some(user_id) = &conn.user_id {
            if let Some(mut connections) = state.user_connections.get_mut(user_id) {
                connections.retain(|id| id != connection_id);
                if connections.is_empty() {
                    state.user_connections.remove(user_id);
                }
            }
        }

        // 广播用户断开连接消息
        if let Some(user_id) = conn.user_id {
            let disconnect_msg = Message {
                id: Uuid::new_v4().to_string(),
                msg_type: MessageType::UserDisconnected,
                payload: serde_json::json!({"user_id": user_id}),
                timestamp: chrono::Utc::now().timestamp() as u64,
                user_id: Some(user_id),
            };
            let _ = state.broadcast_tx.send(disconnect_msg);
        }
    }
}
