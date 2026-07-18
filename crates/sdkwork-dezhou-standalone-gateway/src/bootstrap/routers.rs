use axum::Router;

use sdkwork_dezhou_gateway_assembly::{
    assemble_application_business_router_with_service, with_dezhou_app_request_context,
    SharedTableService,
};
use sdkwork_routes_health_app_api::build_health_router;

pub async fn build_router(table_service: SharedTableService) -> Router {
    let business = assemble_application_business_router_with_service(table_service).router;
    build_router_from_business(business)
}

pub fn build_router_from_business(business: Router) -> Router {
    Router::new()
        .merge(with_dezhou_app_request_context(build_health_router()))
        .merge(business)
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_DEZHOU_ENVIRONMENT"],
            &[
                "SDKWORK_DEZHOU_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ))
}
