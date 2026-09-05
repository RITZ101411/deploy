use axum::{Router, http::StatusCode, routing::get};
use kube::Client;

pub fn routes() -> Router<Client> {
    Router::new().route("/health", get(health))
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}
