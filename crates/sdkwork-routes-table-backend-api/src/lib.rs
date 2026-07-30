mod http_route_manifest;
mod paths;
mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use routes::{build_table_backend_router, DezhouTableStore};

pub async fn gateway_mount<R>(store: DezhouTableStore<R>) -> axum::Router
where
    R: sdkwork_dezhou_table_service::DezhouTableRepository + Send + Sync + 'static,
{
    build_table_backend_router(store)
}
