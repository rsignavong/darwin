use super::StatusImportation;
use crate::resources::{ImportInstanceId, ImportationProgression, ProcessorId};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, new)]
pub struct Status {
    pub processor_id: Arc<ProcessorId>,
    #[new(default)]
    pub import_instance_id: Option<Arc<ImportInstanceId>>,
    #[new(default)]
    pub importation: StatusImportation,
    #[new(default)]
    pub progression: Arc<ImportationProgression>,
}
