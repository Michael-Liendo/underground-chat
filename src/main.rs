use std::env;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower_http::cors::CorsLayer;

mod routes;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: String,
    room: String,
    username: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Join {
    room: String,
    username: String,
}

fn on_connect(socket: SocketRef) {
    socket.on("join", |socket: SocketRef, Data::<Join>(join)| async move {
        socket.leave_all().ok();
        socket.join(join.room.clone()).ok();

        socket
            .within(join.room.clone())
            .broadcast()
            .emit("join", join)
            .ok();
    });

    socket.on(
        "chat message",
        |socket: SocketRef, Data::<Message>(data)| {
            println!("Received event: {:?} \n", data);

            let chat_id = &data.room;
            socket
                .within(chat_id.to_string())
                .broadcast()
                .emit("chat message", data)
                .ok();
        },
    );
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_port = env::var("PORT").expect("PORT must be set.");

    let origins_urls: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .expect("CORS_ORIGINS must be set.")
        .split(",")
        .map(|url| url.parse::<HeaderValue>().unwrap())
        .collect();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/ws", on_connect);

    let app = Router::new()
        .nest("/api", routes::routes())
        .layer(layer)
        .layer(
            CorsLayer::new()
                .allow_origin(origins_urls)
                .allow_headers([header::CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", server_port))
        .await
        .unwrap();

    println!(
        "->> LISTENING ON http://{:?}\n",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
