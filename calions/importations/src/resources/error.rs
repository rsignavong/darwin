use crate::resources::CsvColumn;
use anyhow::Error as AnyError;
use csv::Error as CsvError;
use rusoto_core::RusotoError;
use rusoto_s3::GetObjectError;
use std::convert::Infallible;
use thiserror::Error;
use ulid::MonotonicError as UlidError;

#[derive(Debug, Error)]
pub enum ResourcesError {
    #[error("CsvSeparatorByte: {0}")]
    CsvSeparatorByte(String),
    #[error("ImportationCsvRecord")]
    ImportationCsvRecord(#[from] CsvError),
    #[error("ImportationProducer")]
    ImportationProducer { source: AnyError },
    #[error("ImportationS3GetObject")]
    ImportationS3GetObject(#[from] RusotoError<GetObjectError>),
    #[error("ImportationS3NoObject")]
    ImportationS3NoObject,
    #[error("RecordIdGeneration {0}")]
    RecordIdGeneration(UlidError),
    #[error("RecordValueFromStr")]
    RecordValueFromStr { source: Infallible },
    #[error("RecordValueNotFound: {0}")]
    RecordValueNotFound(CsvColumn),
}
