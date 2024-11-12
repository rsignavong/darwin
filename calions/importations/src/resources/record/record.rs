use super::{RecordFile, RecordImportInstance};
use crate::processors::StatusProcessorSender;
use crate::resources::{ImportationFilesProgressions, MappingId, RecordData, RecordId};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct Record {
    pub file: Arc<RecordFile>,
    pub files_progresions: Arc<ImportationFilesProgressions>,
    pub id: Arc<RecordId>,
    pub import_instance: Arc<RecordImportInstance>,
    pub mapping_id: Arc<MappingId>,
    pub record_data: Arc<RecordData>,
    pub status_processor_sender: StatusProcessorSender,
}
