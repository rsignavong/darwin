use crate::processors::StatusMsg;
use crate::resources::{CsvLineCount, Record, RecordData, RecordId};
use crate::resources::{ImportationProgression, MappingId};
use crate::resources::{RecordImportInstanceProgression, RecordSource};
use anyhow::{anyhow, Error as AnyError};
use data_stream::processor::ProcessorMessage;
use data_stream::stream::{InputEvent, InputMessage, OutputStream, StreamEvent};
use log::warn;
use serde::Serialize;
use smol::block_on;
use std::collections::HashMap;
use std::sync::{atomic::Ordering, Arc};

#[derive(Debug, Serialize)]
pub struct RecordMetadata {
    activated_mapping_id: Arc<MappingId>,
    import_instance: Arc<RecordImportInstanceProgression>,
    source: RecordSource,
}

#[derive(Debug, Serialize)]
pub struct RecordBody {
    id: Arc<RecordId>,
    record: Arc<RecordData>,
}

impl OutputStream<RecordBody, RecordMetadata> for Record {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, RecordBody> {
        let record_body = RecordBody {
            id: self.id.clone(),
            record: self.record_data.clone(),
        };

        let mut body = HashMap::new();
        body.insert(1, record_body);
        body
    }

    fn metadata(&self) -> Option<RecordMetadata> {
        self.files_progresions.fetch_add(1, Ordering::AcqRel);
        let total_lines_read = CsvLineCount::from(self.files_progresions.load(Ordering::Acquire));

        let progression: Arc<ImportationProgression> = Arc::new(
            (*total_lines_read as f64 * 100.0 / **self.import_instance.total_lines as f64).into(),
        );

        let import_instance = Arc::new(RecordImportInstanceProgression::new(
            self.import_instance.id.to_owned(),
            progression,
            total_lines_read.eq(&self.import_instance.total_lines),
        ));

        let status_sent = block_on(self.status_processor_sender.send(ProcessorMessage::Input(
            InputMessage::new(
                InputEvent::Custom,
                StatusMsg::Progression(import_instance.clone()),
                None,
            ),
        )));

        if let Err(e) = status_sent {
            warn!("Unable to update status progression: {:?}", anyhow!(e));
        }

        Some(RecordMetadata {
            activated_mapping_id: self.mapping_id.clone(),
            import_instance,
            source: RecordSource::Importation(self.file.clone()),
        })
    }
}
