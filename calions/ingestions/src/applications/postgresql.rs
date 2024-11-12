use crate::Settings;
use deadpool_postgres::PoolError;
use deadpool_postgres::{Client, Manager, Pool};
use log::info;
use tokio_postgres::{config::Config, NoTls};

#[derive(Clone)]
pub struct PostgreSql(Pool);

impl PostgreSql {
    pub async fn client(&self) -> Result<Client, PoolError> {
        self.0.get().await
    }

    pub fn new() -> Self {
        let pg = &Settings::get().postgresql;
        let mut cfg = Config::new();
        cfg.host(&pg.hostname);
        cfg.user(&pg.username);
        cfg.password(&pg.password);
        cfg.dbname(&pg.database);
        cfg.port(pg.port);
        info!("{:#?}", cfg);
        let mgr = Manager::new(cfg, NoTls);
        let pool = Pool::new(mgr, pg.pool_size);

        PostgreSql(pool)
    }
}
