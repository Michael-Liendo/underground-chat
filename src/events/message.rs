use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};

#[derive(Debug, Deserialize, Serialize)]

pub struct Message {
    content: String,
    room: String,
    username: String,
    created_at: DateTime<Utc>,
}

pub fn on_chat_message(socket: SocketRef, Data(data): Data<Message>) {
    println!("Received event: {:?} \n", data);

    let chat_id = &data.room;
    socket
        .within(chat_id.to_string())
        .broadcast()
        .emit("chat message", data)
        .ok();
}
