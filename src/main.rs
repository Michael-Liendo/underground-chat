use std::env;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use dotenv::dotenv;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower_http::cors::CorsLayer;

mod routes;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Message {
    content: String,
    room: String,
    username: String,
}

fn on_connect(socket: SocketRef) {
    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room)| async move {
            socket.leave_all().ok();
            socket.join(room.clone()).ok();
            // todo: emit a event with the user name
        },
    );

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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "->> LISTENING ON http://{:?}\n",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
