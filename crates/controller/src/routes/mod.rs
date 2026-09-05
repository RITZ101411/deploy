use axum::Router;
use kube::Client;

pub mod apps;
pub mod health;

pub fn router() -> Router<Client> {
    Router::new()
        .merge(health::routes())
        .merge(apps::routes())
}
