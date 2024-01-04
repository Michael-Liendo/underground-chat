use std::env;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use dotenv::dotenv;
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower_http::cors::CorsLayer;

mod routes;

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    println!("\nSocket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // todo: emit a event with the user name
    println!("Auth data {:?}", data);

    socket.on("chat message", |socket: SocketRef, Data::<Value>(data)| {
        println!("Received event: {:?} \n", data);

        let chat_id = &data.as_array().unwrap()[2].as_str().unwrap();
        socket
            .within(chat_id.to_string())
            .emit("chat message", &data)
            .ok();
    });
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
