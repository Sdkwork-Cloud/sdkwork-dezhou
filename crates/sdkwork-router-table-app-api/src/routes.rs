use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use sdkwork_dezhou_table_service::{DezhouTableQuery, DezhouTableRepository, DezhouTableService};
use sdkwork_web_axum::RequirePrincipal;
use std::sync::Arc;

use crate::error::{map_dezhou_error, ok_envelope};

pub type DezhouTableStore<R> = Arc<DezhouTableService<R>>;

#[derive(Debug, serde::Deserialize, Default)]
pub struct DezhouListQuery {
    page: Option<u32>,
    page_size: Option<u32>,
    status: Option<String>,
}

pub fn build_table_app_router<R>(store: DezhouTableStore<R>) -> Router
where
    R: DezhouTableRepository + Send + Sync + 'static,
{
    Router::new()
        .route(crate::paths::TABLES_LIST_PATH, get(list_tables::<R>))
        .route(crate::paths::TABLE_DETAIL_PATH, get(get_table::<R>))
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
    respond_list(store.as_ref(), principal.tenant_id(), query).await
}

async fn get_table<R>(
    RequirePrincipal(principal): RequirePrincipal,
    State(store): State<DezhouTableStore<R>>,
    Path(table_id): Path<String>,
) -> Response
where
    R: DezhouTableRepository + Send + Sync,
{
    let tenant_id = principal.tenant_id();
    match store.get_table(tenant_id, &table_id).await {
        Ok(item) => (axum::http::StatusCode::OK, ok_envelope(item)).into_response(),
        Err(error) => {
            let (status, problem) = map_dezhou_error(error);
            (status, problem).into_response()
        }
    }
}

pub async fn respond_list<R>(
    store: &DezhouTableService<R>,
    tenant_id: &str,
    query: DezhouListQuery,
) -> Response
where
    R: DezhouTableRepository + Send + Sync,
{
    let table_query = DezhouTableQuery {
        page: query.page,
        page_size: query.page_size,
        status: query.status,
    };

    match store.list_tables(tenant_id, table_query).await {
        Ok(page) => (axum::http::StatusCode::OK, ok_envelope(page)).into_response(),
        Err(error) => {
            let (status, problem) = map_dezhou_error(error);
            (status, problem).into_response()
        }
    }
}
