use crate::resources::{CsvLineCount, ImportInstanceId};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct RecordImportInstance {
    pub id: Arc<ImportInstanceId>,
    pub total_lines: Arc<CsvLineCount>,
}
