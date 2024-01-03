use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct ChatCreate {
    success: bool,
    data: Option<Chat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatForCreate {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chat {
    id: String,
    title: String,
}

impl Chat {
    fn new(chat: ChatForCreate) -> Self {
        Chat {
            id: Uuid::new_v4().to_string(),
            title: chat.title,
        }
    }
}

pub fn chats_routes() -> Router {
    Router::new()
        .route("/create", post(create_chat))
        .route("/ws", get(upgrade_sockets))
}

async fn upgrade_sockets(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn create_chat(
    Json(chat_for_create): Json<ChatForCreate>,
) -> Result<(StatusCode, Json<ChatCreate>), String> {
    let new_chat = Chat::new(chat_for_create);
    Ok((
        StatusCode::CREATED,
        Json(ChatCreate {
            success: true,
            data: Some(new_chat),
        }),
    ))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
