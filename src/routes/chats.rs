use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn chats_routes() -> Router {
    Router::new().route("/create", post(create_chat))
}

async fn create_chat(Json(chat_for_create): Json<ChatForCreate>) -> Result<Json<Chat>, String> {
    let new_chat = Chat::new(chat_for_create);
    Ok(Json(new_chat))
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
