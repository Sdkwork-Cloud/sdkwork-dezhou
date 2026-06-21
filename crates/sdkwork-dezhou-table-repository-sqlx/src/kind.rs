use async_trait::async_trait;
use sdkwork_dezhou_table_service::{
    DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery, DezhouTableRepository,
};

use crate::memory::InMemoryDezhouTableRepository;
use crate::sqlx::SqlxDezhouTableRepository;

#[derive(Clone)]
pub enum DezhouTableRepositoryKind {
    Memory(InMemoryDezhouTableRepository),
    Sqlx(Box<SqlxDezhouTableRepository>),
}

#[async_trait]
impl DezhouTableRepository for DezhouTableRepositoryKind {
    async fn list_tables(
        &self,
        tenant_id: &str,
        query: &DezhouTableQuery,
    ) -> DezhouResult<DezhouTablePage> {
        match self {
            Self::Memory(repo) => repo.list_tables(tenant_id, query).await,
            Self::Sqlx(repo) => repo.list_tables(tenant_id, query).await,
        }
    }

    async fn get_table_item(
        &self,
        tenant_id: &str,
        table_id: &str,
    ) -> DezhouResult<DezhouTableItem> {
        match self {
            Self::Memory(repo) => repo.get_table_item(tenant_id, table_id).await,
            Self::Sqlx(repo) => repo.get_table_item(tenant_id, table_id).await,
        }
    }
}
