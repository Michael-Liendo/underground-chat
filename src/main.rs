use std::env;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use dotenv::dotenv;
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;

mod events;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_port = env::var("PORT").expect("PORT must be set.");
    let server_host = env::var("HOST").expect("HOST must be set.");

    let origins_urls: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .expect("CORS_ORIGINS must be set.")
        .split(",")
        .map(|url| url.parse::<HeaderValue>().unwrap())
        .collect();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/ws", events::on_connect);

    let app = Router::new()
        .nest("/api", routes::routes())
        .layer(layer)
        .layer(
            CorsLayer::new()
                .allow_origin(origins_urls)
                .allow_headers([header::CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_host, server_port))
        .await
        .unwrap();

    println!(
        "->> LISTENING ON http://{:?}\n",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
