use crate::resources::{CsvColumn, CsvHeader, CsvLineCount, CsvSeparator};
use crate::resources::{ImportFileId, ImportId, ImportInstanceId, ImportName};
use crate::resources::{MappingField, MappingId, ProcessorId, ProcessorTopic};
use crate::resources::{S3BucketKey, S3BucketName, S3Endpoint, S3RegionName};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportInstanceMetadata;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub activated_mapping_id: Arc<MappingId>,
    pub files: Arc<Vec<ImportFile>>,
    pub import: Arc<Import>,
    pub mapping: Arc<Vec<Mapping>>,
    pub producer_topic: Arc<ProcessorTopic>,
    pub storage: Storage,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Import {
    pub header: Arc<CsvHeader>,
    pub id: Arc<ImportId>,
    pub name: Arc<ImportName>,
    pub separator: Arc<CsvSeparator>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImportFile {
    pub id: ImportFileId,
    pub lines: CsvLineCount,
    pub path: S3BucketKey,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mapping {
    pub column: CsvColumn,
    pub field_alias: MappingField,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub bucket: Arc<S3BucketName>,
    pub endpoint: Arc<S3Endpoint>,
    pub region_name: Option<Arc<S3RegionName>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImportInstanceBody {
    pub config: Config,
    pub id: Arc<ImportInstanceId>,
    pub processor_id: Arc<ProcessorId>,
}
