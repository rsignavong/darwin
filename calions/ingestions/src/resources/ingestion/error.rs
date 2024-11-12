use crate::resources::{ContactError, GdprError};
use data_stream::consumer::ConsumerError;
use deadpool_postgres::PoolError;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("ActivatedMappingsConsumerSpawn")]
    ActivatedMappingsConsumerSpawn { source: ConsumerError },
    #[error("Contact")]
    Contact(#[from] ContactError),
    #[error("Gdpr")]
    Gdpr(#[from] GdprError),
    #[error("IngestionAnonymizationsConsumerSpawn")]
    IngestionAnonymizationsConsumerSpawn { source: ConsumerError },
    #[error("IngestionAnonymizationPostgresTransactionNew")]
    IngestionAnonymizationPostgresTransactionNew { source: PostgresError },
    #[error("IngestionAnonymizationPostgresTransactionUncommitted")]
    IngestionAnonymizationPostgresTransactionUncommitted { source: PostgresError },
    #[error("IngestionCreateMissingActivatedMappingId")]
    IngestionCreateMissingActivatedMappingId,
    #[error("IngestionCreatePostgresTransactionNew")]
    IngestionCreatePostgresTransactionNew { source: PostgresError },
    #[error("IngestionCreatePostgresTransactionUncommitted")]
    IngestionCreatePostgresTransactionUncommitted { source: PostgresError },
    #[error("IngestionMissingActivatedMappingId")]
    IngestionMissingActivatedMappingId,
    #[error("IngestionMissingSelfProcessorSender")]
    IngestionMissingSelfProcessorSender,
    #[error("IngestionReconciliationRecordsConsumerSpawn")]
    IngestionReconciliationRecordsConsumerSpawn { source: ConsumerError },
    #[error("IngestionSetMappingMissingActivatedMappingId")]
    IngestionSetMappingMissingActivatedMappingId,
    #[error("IngestionUpdateNoContact")]
    IngestionUpdateNoContact,
    #[error("IngestionUpdatePostgresTransactionNew")]
    IngestionUpdatePostgresTransactionNew { source: PostgresError },
    #[error("IngestionUpdatePostgresTransactionUncommitted")]
    IngestionUpdatePostgresTransactionUncommitted { source: PostgresError },
    #[error("IngestionUpdateMissingActivatedMappingId")]
    IngestionUpdateMissingActivatedMappingId,
    #[error("PostgreSqlClient")]
    PostgreSqlClient(#[from] PoolError),
}
