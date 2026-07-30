mod error;
mod http_route_manifest;
mod paths;
mod routes;

pub use error::{map_dezhou_error, ok_envelope};
pub use http_route_manifest::gateway_route_manifest;
pub use paths::{TABLES_LIST_PATH, TABLE_DETAIL_PATH};
pub use routes::{build_table_app_router, respond_list, DezhouListQuery, DezhouTableStore};

pub async fn gateway_mount<R>(store: DezhouTableStore<R>) -> axum::Router
where
    R: sdkwork_dezhou_table_service::DezhouTableRepository + Send + Sync + 'static,
{
    build_table_app_router(store)
}
