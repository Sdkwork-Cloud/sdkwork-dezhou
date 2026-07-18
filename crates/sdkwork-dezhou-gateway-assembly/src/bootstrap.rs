//! Gateway bootstrap for sdkwork-dezhou.

use axum::Router;
use sdkwork_dezhou_service_host::{build_table_service, SharedTableService};
use sdkwork_routes_table_app_api::build_table_app_router;
use sdkwork_routes_table_backend_api::build_table_backend_router;

use crate::web_bootstrap::{with_dezhou_app_request_context, with_dezhou_backend_request_context};

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let service = build_table_service().await?;
    Ok(assemble_application_business_router_with_service(service))
}

pub fn assemble_application_business_router_with_service(
    service: SharedTableService,
) -> ApplicationAssembly {
    let app = with_dezhou_app_request_context(build_table_app_router(service.clone()));
    let backend = with_dezhou_backend_request_context(build_table_backend_router(service));
    ApplicationAssembly {
        router: Router::new().merge(app).merge(backend),
    }
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}
