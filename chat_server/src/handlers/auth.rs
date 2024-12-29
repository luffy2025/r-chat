use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;

pub(crate) async fn signup_handler(State(state): State<AppState>) -> impl IntoResponse {
    format!("signup {:?}", state.config.server.port) // "signup"

    // "signup"
}

pub(crate) async fn signin_handler() -> impl IntoResponse {
    "signin"
}
