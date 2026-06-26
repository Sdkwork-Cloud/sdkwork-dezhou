use async_trait::async_trait;
use sdkwork_dezhou_table_service::{
    DezhouError, DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery,
    DezhouTableRepository,
};

#[derive(Default, Clone)]
pub struct InMemoryDezhouTableRepository {
    items: Vec<DezhouTableItem>,
}

impl InMemoryDezhouTableRepository {
    pub fn with_seed(items: Vec<DezhouTableItem>) -> Self {
        Self { items }
    }
}

#[async_trait]
impl DezhouTableRepository for InMemoryDezhouTableRepository {
    async fn list_tables(
        &self,
        _tenant_id: &str,
        query: &DezhouTableQuery,
    ) -> DezhouResult<DezhouTablePage> {
        let filtered: Vec<DezhouTableItem> = if let Some(status) = &query.status {
            self.items
                .iter()
                .filter(|item| item.status == *status)
                .cloned()
                .collect()
        } else {
            self.items.clone()
        };

        let total = filtered.len() as u64;
        let offset = query.offset() as usize;
        let limit = query.limit() as usize;
        let page_items = filtered.into_iter().skip(offset).take(limit).collect();

        Ok(DezhouTablePage {
            items: page_items,
            total,
            page: query.page.unwrap_or(1),
            page_size: query.limit(),
        })
    }

    async fn get_table_item(
        &self,
        _tenant_id: &str,
        table_id: &str,
    ) -> DezhouResult<DezhouTableItem> {
        self.items
            .iter()
            .find(|item| item.id == table_id)
            .cloned()
            .ok_or_else(|| DezhouError::not_found(format!("table {table_id} not found")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_dezhou_table_service::DezhouTableQuery;

    #[tokio::test]
    async fn list_tables_paginates_items() {
        let repo = InMemoryDezhouTableRepository::with_seed(vec![DezhouTableItem {
            id: "t1".into(),
            table_code: "holdem-01".into(),
            title: "Texas Hold'em Table 1".into(),
            summary: None,
            max_seats: Some(9),
            current_seats: Some(0),
            status: "open".into(),
        }]);

        let page = repo
            .list_tables("100001", &DezhouTableQuery::default())
            .await
            .expect("page");

        assert_eq!(page.total, 1);
        assert_eq!(page.items[0].title, "Texas Hold'em Table 1");
    }
}
