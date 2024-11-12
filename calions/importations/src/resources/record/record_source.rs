use super::RecordFile;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RecordSource {
    Importation(Arc<RecordFile>),
}
