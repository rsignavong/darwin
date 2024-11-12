use crate::resources::{IngestionData, IngestionDataRecord, ProcessorId};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub type IngestionDataBody = IngestionData;

#[derive(Debug, Serialize)]
pub struct IngestionDataMetadata {
    processor_id: Arc<ProcessorId>,
}

impl OutputStream<IngestionDataBody, IngestionDataMetadata> for IngestionDataRecord {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.processor_id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, IngestionDataBody> {
        let ingestion_data_body = self.data.clone();

        let mut body = HashMap::new();
        body.insert(1, ingestion_data_body);
        body
    }

    fn metadata(&self) -> Option<IngestionDataMetadata> {
        Some(IngestionDataMetadata {
            processor_id: self.processor_id.clone(),
        })
    }
}
