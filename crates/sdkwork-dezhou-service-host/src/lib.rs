use std::sync::Arc;

use sdkwork_database_sqlx::DatabasePool;
use sdkwork_dezhou_database_host::{
    bootstrap_dezhou_database, bootstrap_dezhou_database_from_env, DezhouDatabaseHost,
};
use sdkwork_dezhou_table_repository_sqlx::{DezhouTableRepositoryKind, SqlxDezhouTableRepository};
use sdkwork_dezhou_table_service::DezhouTableService;

pub type SharedTableService = Arc<DezhouTableService<DezhouTableRepositoryKind>>;

pub struct DezhouServiceHost {
    service: SharedTableService,
    database: DezhouDatabaseHost,
}

impl DezhouServiceHost {
    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_dezhou_database_from_env().await?;
        Ok(Self::from_database(database))
    }

    pub async fn from_pool(pool: DatabasePool) -> Result<Self, String> {
        let database = bootstrap_dezhou_database(pool).await?;
        Ok(Self::from_database(database))
    }

    fn from_database(database: DezhouDatabaseHost) -> Self {
        let repository = DezhouTableRepositoryKind::Sqlx(Box::new(SqlxDezhouTableRepository::new(
            database.pool().clone(),
        )));
        Self {
            service: Arc::new(DezhouTableService::new(repository)),
            database,
        }
    }

    pub fn service(&self) -> SharedTableService {
        self.service.clone()
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }
}

pub async fn build_table_service() -> Result<SharedTableService, String> {
    let host = DezhouServiceHost::from_env().await?;
    Ok(host.service())
}
