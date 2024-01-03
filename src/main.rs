use axum::Router;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", routes::routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(
        "->> LISTENING ON http://{:?}\n",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
