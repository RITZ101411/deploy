use axum::Router;

pub mod health;

pub fn router() -> Router {
    Router::new()
        .merge(health::routes())
}
