use async_trait::async_trait;

use crate::domain::models::{DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery};

#[async_trait]
pub trait DezhouTableRepository: Send + Sync {
    async fn list_tables(
        &self,
        tenant_id: &str,
        query: &DezhouTableQuery,
    ) -> DezhouResult<DezhouTablePage>;

    async fn get_table_item(
        &self,
        tenant_id: &str,
        table_id: &str,
    ) -> DezhouResult<DezhouTableItem>;
}
