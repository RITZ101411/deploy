use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::routes;

#[derive(OpenApi)]
#[openapi(paths(crate::routes::health::health))]
struct ApiDoc;

pub fn create() -> Router {
    Router::new()
        .merge(routes::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
}
