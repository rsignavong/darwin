use crate::resources::{ProcessorId, ReconciliationData, ReconciliationDataRecord};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub type ReconciliationDataBody = ReconciliationData;

#[derive(Debug, Serialize)]
pub struct ReconciliationDataMetadata {
    processor_id: Arc<ProcessorId>,
}

impl OutputStream<ReconciliationDataBody, ReconciliationDataMetadata> for ReconciliationDataRecord {
    fn key(&self) -> Result<String, AnyError> {
        let key = match self.data {
            ReconciliationData::Mapping(ref mapping) => mapping.field.to_string(),
            ReconciliationData::Profile(ref profile) => profile.log.key.to_string(),
            _ => self.processor_id.to_string(),
        };

        Ok(key)
    }

    fn event(&self) -> StreamEvent {
        match self.data {
            ReconciliationData::Profile(ref profile) => {
                if profile.log.old.is_some() {
                    StreamEvent::Updated
                } else {
                    StreamEvent::Created
                }
            }
            _ => StreamEvent::Created,
        }
    }

    fn body(&self) -> HashMap<u64, ReconciliationDataBody> {
        let reconciliation_data_body = self.data.clone();

        let mut body = HashMap::new();
        body.insert(1, reconciliation_data_body);
        body
    }

    fn metadata(&self) -> Option<ReconciliationDataMetadata> {
        Some(ReconciliationDataMetadata {
            processor_id: self.processor_id.clone(),
        })
    }
}
