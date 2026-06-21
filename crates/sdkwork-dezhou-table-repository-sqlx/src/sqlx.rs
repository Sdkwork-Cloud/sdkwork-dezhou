use async_trait::async_trait;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_dezhou_table_service::{
    DezhouError, DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery,
    DezhouTableRepository,
};
use sdkwork_utils_rust::string::is_blank;

#[derive(Clone)]
pub struct SqlxDezhouTableRepository {
    pool: DatabasePool,
}

impl SqlxDezhouTableRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DezhouTableRepository for SqlxDezhouTableRepository {
    async fn list_tables(
        &self,
        tenant_id: &str,
        query: &DezhouTableQuery,
    ) -> DezhouResult<DezhouTablePage> {
        if is_blank(Some(tenant_id)) {
            return Err(DezhouError::invalid("tenant_id is required"));
        }

        let limit = query.limit() as i64;
        let offset = query.offset() as i64;
        let status = query.status.as_deref();

        match &self.pool {
            DatabasePool::Postgres(pool, _) => {
                list_postgres(pool, tenant_id, status, limit, offset).await
            }
            DatabasePool::Sqlite(pool, _) => {
                list_sqlite(pool, tenant_id, status, limit, offset).await
            }
        }
    }

    async fn get_table_item(
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

        match &self.pool {
            DatabasePool::Postgres(pool, _) => get_postgres(pool, tenant_id, table_id).await,
            DatabasePool::Sqlite(pool, _) => get_sqlite(pool, tenant_id, table_id).await,
        }
    }
}

async fn list_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> DezhouResult<DezhouTablePage> {
    let rows = if let Some(status) = status {
        sqlx::query_as::<_, TableRow>(
            "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
             WHERE tenant_id = $1 AND deleted_at IS NULL AND status = $2 \
             ORDER BY title ASC LIMIT $3 OFFSET $4",
        )
        .bind(tenant_id)
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, TableRow>(
            "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
             WHERE tenant_id = $1 AND deleted_at IS NULL \
             ORDER BY title ASC LIMIT $2 OFFSET $3",
        )
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
    .map_err(map_sqlx_error)?;

    let total: i64 = if let Some(status) = status {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM dezhou_table WHERE tenant_id = $1 AND deleted_at IS NULL AND status = $2",
        )
        .bind(tenant_id)
        .bind(status)
        .fetch_one(pool)
        .await
    } else {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM dezhou_table WHERE tenant_id = $1 AND deleted_at IS NULL",
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await
    }
    .map_err(map_sqlx_error)?;

    Ok(DezhouTablePage {
        items: rows.into_iter().map(TableRow::into_item).collect(),
        total: total as u64,
        page: ((offset / limit.max(1)) + 1) as u32,
        page_size: limit as u32,
    })
}

async fn get_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    table_id: &str,
) -> DezhouResult<DezhouTableItem> {
    let row = sqlx::query_as::<_, TableRow>(
        "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
         WHERE tenant_id = $1 AND deleted_at IS NULL AND (id = $2 OR table_code = $2) LIMIT 1",
    )
    .bind(tenant_id)
    .bind(table_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| DezhouError::not_found(format!("table {table_id} not found")))?;

    Ok(row.into_item())
}

async fn list_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> DezhouResult<DezhouTablePage> {
    let rows = if let Some(status) = status {
        sqlx::query_as::<_, TableRow>(
            "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
             WHERE tenant_id = ? AND deleted_at IS NULL AND status = ? \
             ORDER BY title ASC LIMIT ? OFFSET ?",
        )
        .bind(tenant_id)
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, TableRow>(
            "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
             WHERE tenant_id = ? AND deleted_at IS NULL \
             ORDER BY title ASC LIMIT ? OFFSET ?",
        )
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
    .map_err(map_sqlx_error)?;

    let total: i64 = if let Some(status) = status {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM dezhou_table WHERE tenant_id = ? AND deleted_at IS NULL AND status = ?",
        )
        .bind(tenant_id)
        .bind(status)
        .fetch_one(pool)
        .await
    } else {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM dezhou_table WHERE tenant_id = ? AND deleted_at IS NULL",
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await
    }
    .map_err(map_sqlx_error)?;

    Ok(DezhouTablePage {
        items: rows.into_iter().map(TableRow::into_item).collect(),
        total: total as u64,
        page: ((offset / limit.max(1)) + 1) as u32,
        page_size: limit as u32,
    })
}

async fn get_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    table_id: &str,
) -> DezhouResult<DezhouTableItem> {
    let row = sqlx::query_as::<_, TableRow>(
        "SELECT id, table_code, title, summary, max_seats, current_seats, status FROM dezhou_table \
         WHERE tenant_id = ? AND deleted_at IS NULL AND (id = ? OR table_code = ?) LIMIT 1",
    )
    .bind(tenant_id)
    .bind(table_id)
    .bind(table_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| DezhouError::not_found(format!("table {table_id} not found")))?;

    Ok(row.into_item())
}

#[derive(sqlx::FromRow)]
struct TableRow {
    id: String,
    table_code: String,
    title: String,
    summary: Option<String>,
    max_seats: i32,
    current_seats: i32,
    status: String,
}

impl TableRow {
    fn into_item(self) -> DezhouTableItem {
        DezhouTableItem {
            id: self.id,
            table_code: self.table_code,
            title: self.title,
            summary: self.summary,
            max_seats: Some(self.max_seats as u32),
            current_seats: Some(self.current_seats as u32),
            status: self.status,
        }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> DezhouError {
    DezhouError::invalid(error.to_string())
}
