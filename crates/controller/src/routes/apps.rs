use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use kube::Client;
use serde::{Deserialize, Serialize};

use crate::k8s::deployment::create_app_deployment;

#[derive(Debug, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub image: String,
    #[serde(default = "default_port")]
    pub port: i32,
}

fn default_port() -> i32 {
    80
}

#[derive(Debug, Serialize)]
pub struct CreateAppResponse {
    pub name: String,
    pub status: String,
}

pub fn routes() -> Router<Client> {
    Router::new().route("/apps", post(create_app))
}

async fn create_app(
    State(client): State<Client>,
    Json(req): Json<CreateAppRequest>,
) -> impl IntoResponse {
    match create_app_deployment(client, &req.name, &req.image, req.port).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(CreateAppResponse {
                name: req.name,
                status: "created".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("failed to create deployment: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    }
}
