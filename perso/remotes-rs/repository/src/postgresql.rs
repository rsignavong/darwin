use crate::{RepositoryConfig, RepositoryError};
use deadpool::managed::{Object, PoolConfig};
use deadpool_postgres::config::{Config, ManagerConfig, RecyclingMethod};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;

pub type PgConn = Object<Manager>;

#[derive(Clone)]
pub struct PostgreSql(Pool);

impl PostgreSql {
    pub async fn pool(&self) -> Result<PgConn, RepositoryError> {
        Ok(self.0.get().await?)
    }

    pub fn new(config: &RepositoryConfig) -> Result<Self, RepositoryError> {
        let mut cfg = Config::new();
        cfg.dbname = Some(config.database);
        cfg.user = Some(config.username);
        cfg.password = Some(config.password);
        cfg.host = Some(config.hostname);
        cfg.port = Some(config.port);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg.pool = Some(PoolConfig::new(config.pool_size));

        let pool = cfg.create_pool(NoTls)?;

        Ok(PostgreSql(pool))
    }
}
