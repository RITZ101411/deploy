use axum::Router;
use kube::Client;
use tower_http::trace::TraceLayer;

use crate::routes;

pub fn create(client: Client) -> Router {
    routes::router()
        .layer(TraceLayer::new_for_http())
        .with_state(client)
}
