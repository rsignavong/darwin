use crate::processors::StatusProcessorSender;
use crate::resources::{ImportInstanceId, RecordImportInstanceProgression};
use data_stream::stream::InputStream;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum StatusMsg {
    Importing(Arc<ImportInstanceId>, StatusProcessorSender),
    Progression(Arc<RecordImportInstanceProgression>),
    Stream,
}

impl InputStream for StatusMsg {}
