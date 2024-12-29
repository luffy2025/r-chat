mod config;
mod handlers;

use axum::routing::{get, patch, post};
use axum::Router;
use handlers::*;
use std::ops::Deref;
use std::sync::Arc;

pub use config::AppConfig;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub config: AppConfig,
}

pub async fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let app = Router::new()
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/messages", get(list_messages_handler));

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", app)
        .with_state(state)
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

impl Deref for AppState {
    type Target = Arc<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
