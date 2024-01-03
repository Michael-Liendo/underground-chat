use axum::Router;

pub mod chats;

pub fn routes() -> Router {
    let routes = Router::new().nest("/chat", chats::chats_routes());

    routes
}
