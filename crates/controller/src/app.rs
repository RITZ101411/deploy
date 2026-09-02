use axum::Router;
use tower_http::trace::TraceLayer;
use crate::routes;

pub fn create() -> Router {
    Router::new()
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
}
