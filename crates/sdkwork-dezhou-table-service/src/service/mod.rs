use sdkwork_utils_rust::id::uuid;
use sdkwork_utils_rust::string::is_blank;

use crate::domain::models::{
    DezhouError, DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery,
};
use crate::ports::repository::DezhouTableRepository;

pub struct DezhouTableService<R: DezhouTableRepository> {
    repository: R,
}

impl<R: DezhouTableRepository> DezhouTableService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn list_tables(
        &self,
        tenant_id: &str,
        query: DezhouTableQuery,
    ) -> DezhouResult<DezhouTablePage> {
        if is_blank(Some(tenant_id)) {
            return Err(DezhouError::invalid("tenant_id is required"));
        }
        self.repository.list_tables(tenant_id, &query).await
    }

    pub async fn get_table(
        &self,
        tenant_id: &str,
        table_id: &str,
    ) -> DezhouResult<DezhouTableItem> {
        if is_blank(Some(tenant_id)) {
            return Err(DezhouError::invalid("tenant_id is required"));
        }
        if is_blank(Some(table_id)) {
            return Err(DezhouError::invalid("table_id is required"));
        }
        self.repository.get_table_item(tenant_id, table_id).await
    }

    pub fn new_table_id() -> String {
        uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::DezhouTableQuery;

    struct EmptyRepo;

    #[async_trait::async_trait]
    impl DezhouTableRepository for EmptyRepo {
        async fn list_tables(
            &self,
            _tenant_id: &str,
            query: &DezhouTableQuery,
        ) -> DezhouResult<DezhouTablePage> {
            Ok(DezhouTablePage {
                items: vec![],
                total: 0,
                page: query.page.unwrap_or(1),
                page_size: query.limit(),
            })
        }

        async fn get_table_item(
            &self,
            _tenant_id: &str,
            _table_id: &str,
        ) -> DezhouResult<DezhouTableItem> {
            Err(DezhouError::not_found("table not found"))
        }
    }

    #[tokio::test]
    async fn list_tables_rejects_empty_tenant() {
        let service = DezhouTableService::new(EmptyRepo);
        let result = service.list_tables("", DezhouTableQuery::default()).await;
        assert_eq!(result.unwrap_err().code(), "invalid");
    }

    #[test]
    fn new_table_id_is_uuid_v4() {
        let id = DezhouTableService::<EmptyRepo>::new_table_id();
        assert_eq!(id.len(), 36);
    }
}
