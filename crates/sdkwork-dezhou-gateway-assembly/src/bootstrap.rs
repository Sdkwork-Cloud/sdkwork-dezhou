//! Gateway bootstrap for sdkwork-dezhou.

use axum::Router;
use sdkwork_dezhou_standalone_gateway::{
    build_table_service, with_dezhou_app_request_context, with_dezhou_backend_request_context,
};
use sdkwork_routes_table_app_api::build_table_app_router;
use sdkwork_routes_table_backend_api::build_table_backend_router;

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let service = build_table_service().await?;
    let app = with_dezhou_app_request_context(build_table_app_router(service.clone()));
    let backend = with_dezhou_backend_request_context(build_table_backend_router(service));
    Ok(ApplicationAssembly {
        router: Router::new().merge(app).merge(backend),
    })
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}
