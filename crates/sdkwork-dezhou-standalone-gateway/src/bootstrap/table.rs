use std::sync::Arc;

use sdkwork_dezhou_database_host::bootstrap_dezhou_database_from_env;
use sdkwork_dezhou_table_repository_sqlx::{
    DezhouTableRepositoryKind, InMemoryDezhouTableRepository, SqlxDezhouTableRepository,
};
use sdkwork_dezhou_table_service::DezhouTableService;

pub type SharedTableService = Arc<DezhouTableService<DezhouTableRepositoryKind>>;

pub async fn build_table_service() -> Result<SharedTableService, String> {
    let mode = std::env::var("DEZHOU_REPOSITORY_MODE").unwrap_or_else(|_| "sqlx".into());
    if mode == "memory" {
        return Ok(Arc::new(DezhouTableService::new(
            DezhouTableRepositoryKind::Memory(InMemoryDezhouTableRepository::with_seed(vec![])),
        )));
    }

    let host = bootstrap_dezhou_database_from_env().await?;
    let repository = DezhouTableRepositoryKind::Sqlx(Box::new(SqlxDezhouTableRepository::new(
        host.pool().clone(),
    )));
    Ok(Arc::new(DezhouTableService::new(repository)))
}
