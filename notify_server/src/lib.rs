mod config;
mod sse;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use sse::*;

pub use config::AppConfig;

const INDEX_HTML: &str = include_str!("../assets/index.html");

pub async fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(sse_handler))
}

pub(crate) async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}
