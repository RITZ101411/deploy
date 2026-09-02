use axum::{Router, http::StatusCode, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn health() -> StatusCode {
    StatusCode::OK
}
