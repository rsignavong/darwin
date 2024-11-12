use crate::resources::{ImportInstanceId, ImportationProgression};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize, new)]
pub struct RecordImportInstanceProgression {
    pub id: Arc<ImportInstanceId>,
    pub progression: Arc<ImportationProgression>,
    pub is_finished: bool,
}
