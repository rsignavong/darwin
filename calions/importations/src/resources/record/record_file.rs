use crate::resources::{CsvLineCount, ImportFileId, S3BucketKey, S3BucketName};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, new)]
pub struct RecordFile {
    pub bucket: Arc<S3BucketName>,
    pub file: Arc<S3BucketKey>,
    pub file_id: ImportFileId,
    pub line: CsvLineCount,
}
