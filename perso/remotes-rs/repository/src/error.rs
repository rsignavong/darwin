use actix_web::ResponseError;
use deadpool_postgres::{config::ConfigError, PoolError};
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use tokio_postgres::Error as TokioPostgresError;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("AdvertiserPostCreatedTryFromSendMail")]
    AdvertiserPostCreatedTryFromSendMail { source: TokioPostgresError },
    #[error("JsonSerialization")]
    JsonSerialization(#[from] SerdeJsonError),
    #[error("PostgresConfig")]
    PostgresConfig(#[from] ConfigError),
    #[error("PostgresPool")]
    PostgresPool(#[from] PoolError),
    #[error("TokioPostgres")]
    TokioPostgres(#[from] TokioPostgresError),
}

impl ResponseError for RepositoryError {}
