use crate::resources::ContactError;
use anyhow::Error as AnyError;
use gdpr::GdprError as GdprRsError;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;
use ulid::{DecodeError as UlidDecodeError, MonotonicError as UlidError};

#[derive(Debug, Error)]
pub enum GdprError {
    #[error("Gdpr")]
    Gdpr(#[from] GdprRsError),
    #[error("GdprEncryptMetadata: {0}")]
    GdprEncryptMetadata(AnyError),
    #[error("GdprKeyAlgoTryFromStr: {0}")]
    GdprKeyAlgoTryFromStr(String),
    #[error("GdprKeyModelContactIdTryFrom")]
    GdprKeyModelContactIdTryFrom { source: ContactError },
    #[error("GdprKeyDataGroupTryFromStr: {0}")]
    GdprKeyDataGroupTryFromStr(String),
    #[error("GdprKeyIdGeneration: {0}")]
    GdprKeyIdGeneration(UlidError),
    #[error("GdprKeyIdTryFromStr: {0}")]
    GdprKeyIdTryFromStr(UlidDecodeError),
    #[error("GdprKeyModelQueryCreate")]
    GdprKeyModelQueryCreate { source: PostgresError },
    #[error("GdprKeyModelQueryDelete")]
    GdprKeyModelQueryDelete { source: PostgresError },
    #[error("GdprKeyModelQueryFetch")]
    GdprKeyModelQueryFetch { source: PostgresError },
    #[error("GdprKeyModelQueryFetchOne")]
    GdprKeyModelQueryFetchOne { source: PostgresError },
    #[error("GdprKeyModelStatementCreate")]
    GdprKeyModelStatementCreate { source: PostgresError },
    #[error("GdprKeyModelStatementDeleteByContactId")]
    GdprKeyModelStatementDeleteByContactId { source: PostgresError },
    #[error("GdprKeyModelStatementFetch")]
    GdprKeyModelStatementFetch { source: PostgresError },
    #[error("GdprKeyModelStatementFetchByContactId")]
    GdprKeyModelStatementFetchByContactId { source: PostgresError },
    #[error("GdprKeyModelTryFromRowAlgo")]
    GdprKeyModelTryFromRowAlgo { source: PostgresError },
    #[error("GdprKeyModelTryFromRowDataGroup")]
    GdprKeyModelTryFromRowDataGroup { source: PostgresError },
    #[error("GdprKeyModelTryFromRowId")]
    GdprKeyModelTryFromRowId { source: PostgresError },
    #[error("GdprKeyModelTryFromRowContactId")]
    GdprKeyModelTryFromRowContactId { source: PostgresError },
    #[error("GdprKeyModelTryFromRowKey")]
    GdprKeyModelTryFromRowKey { source: PostgresError },
    #[error("GdprKeyModelTryFromRowVersion")]
    GdprKeyModelTryFromRowVersion { source: PostgresError },
    #[error("GdprKeyModelTryFromRowInsertedAt")]
    GdprKeyModelTryFromRowInsertedAt { source: PostgresError },
    #[error("GdprKeyModelTryFromRowUpdatedAt")]
    GdprKeyModelTryFromRowUpdatedAt { source: PostgresError },
    #[error("GdprKeyNotCreated")]
    GdprKeyNotCreated,
    #[error("GdprKeyNotDeleted")]
    GdprKeyNotDeleted,
}
