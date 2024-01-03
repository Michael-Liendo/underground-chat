use std::env;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let origins_urls: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .expect("CORS_ORIGINS must be set.")
        .split(",")
        .map(|url| url.parse::<HeaderValue>().unwrap())
        .collect();

    let app = Router::new().nest("/api", routes::routes()).layer(
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
