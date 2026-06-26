use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use sdkwork_dezhou_table_service::{DezhouTableRepository, DezhouTableService};
use sdkwork_routes_table_app_api::DezhouListQuery;
use sdkwork_web_axum::RequirePrincipal;
use std::sync::Arc;

pub type DezhouTableStore<R> = Arc<DezhouTableService<R>>;

pub fn build_table_backend_router<R>(store: DezhouTableStore<R>) -> Router
where
    R: DezhouTableRepository + Send + Sync + 'static,
{
    Router::new()
        .route(crate::paths::TABLES_LIST_PATH, get(list_tables::<R>))
        .with_state(store)
}

async fn list_tables<R>(
    RequirePrincipal(principal): RequirePrincipal,
    State(store): State<DezhouTableStore<R>>,
    Query(query): Query<DezhouListQuery>,
) -> Response
where
    R: DezhouTableRepository + Send + Sync,
{
    sdkwork_routes_table_app_api::respond_list(store.as_ref(), principal.tenant_id(), query).await
}
