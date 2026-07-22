use axum::Router;

use sdkwork_api_dezhou_assembly::{assemble_api_router_with_service, SharedTableService};
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

pub async fn build_router(table_service: SharedTableService) -> Router {
    let business = assemble_api_router_with_service(table_service).router;
    build_router_from_business(business)
}

pub fn build_router_from_business(business: Router) -> Router {
    service_router(business, ServiceRouterConfig::default().with_always_ready()).layer(
        sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_DEZHOU_ENVIRONMENT"],
            &[
                "SDKWORK_DEZHOU_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ),
    )
}
