use axum::{Router, http::StatusCode, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}
