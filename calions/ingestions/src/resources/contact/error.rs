use crate::resources::MappingField;
use std::sync::Arc;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;
use ulid::DecodeError as UlidDecodeError;

#[derive(Debug, Error)]
pub enum ContactError {
    #[error("ContactCountFromRow")]
    ContactCountFromRow { source: PostgresError },
    #[error("ContactIdTryFromStr: {0}")]
    ContactIdTryFromStr(UlidDecodeError),
    #[error("ContactModelQueryCount")]
    ContactModelQueryCount { source: PostgresError },
    #[error("ContactModelQueryCreate")]
    ContactModelQueryCreate { source: PostgresError },
    #[error("ContactModelQueryDelete")]
    ContactModelQueryDelete { source: PostgresError },
    #[error("ContactModelQueryFetch")]
    ContactModelQueryFetch { source: PostgresError },
    #[error("ContactModelQueryFetchOne")]
    ContactModelQueryFetchOne { source: PostgresError },
    #[error("ContactModelQueryUpdate")]
    ContactModelQueryUpdate { source: PostgresError },
    #[error("ContactModelStatementCount ")]
    ContactModelStatementCount { source: PostgresError },
    #[error("ContactModelStatementCreate")]
    ContactModelStatementCreate { source: PostgresError },
    #[error("ContactModelStatementDelete")]
    ContactModelStatementDelete { source: PostgresError },
    #[error("ContactModelStatementFetch")]
    ContactModelStatementFetch { source: PostgresError },
    #[error("ContactModelStatementFetchOne")]
    ContactModelStatementFetchOne { source: PostgresError },
    #[error("ContactModelStatementUpdate")]
    ContactModelStatementUpdate { source: PostgresError },
    #[error("ContactModelTryFromRowData")]
    ContactModelTryFromRowData { source: PostgresError },
    #[error("ContactModelTryFromRowId")]
    ContactModelTryFromRowId { source: PostgresError },
    #[error("ContactModelTryFromRowInsertedAt")]
    ContactModelTryFromRowInsertedAt { source: PostgresError },
    #[error("ContactModelTryFromRowMetadata")]
    ContactModelTryFromRowMetadata { source: PostgresError },
    #[error("ContactModelTryFromRowOrganizationId")]
    ContactModelTryFromRowOrganizationId { source: PostgresError },
    #[error("ContactModelTryFromRowUpdatedAt")]
    ContactModelTryFromRowUpdatedAt { source: PostgresError },
    #[error("ContactModelUpdateDataFromLatestMetadata: {0}")]
    ContactModelUpdateDataFromLatestMetadata(Arc<MappingField>),
    #[error("ContactNotCreated")]
    ContactNotCreated,
    #[error("ContactNotDeleted")]
    ContactNotDeleted,
    #[error("ContactNotUpdated")]
    ContactNotUpdated,
}
