use axum::Router;
use tower_http::cors::CorsLayer;

use crate::bootstrap::table::SharedTableService;
use crate::web_bootstrap::{with_dezhou_app_request_context, with_dezhou_backend_request_context};
use sdkwork_router_health_app_api::build_health_router;
use sdkwork_router_table_app_api::build_table_app_router;
use sdkwork_router_table_backend_api::build_table_backend_router;

pub async fn build_router(table_service: SharedTableService) -> Router {
    let app_routes = Router::new()
        .merge(with_dezhou_app_request_context(build_health_router()))
        .merge(with_dezhou_app_request_context(build_table_app_router(
            table_service.clone(),
        )));

    let backend_routes =
        with_dezhou_backend_request_context(build_table_backend_router(table_service));

    Router::new()
        .merge(app_routes)
        .merge(backend_routes)
        .layer(CorsLayer::permissive())
}
