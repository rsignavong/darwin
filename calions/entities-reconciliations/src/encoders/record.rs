use crate::resources::{AggregateEvent, AggregateId};
use crate::resources::{Entity, MappingId, OrganizationId, ProcessorId};
use crate::resources::{ReconciliationCount, Record};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct RecordBody {
    id: Arc<AggregateId>,
    organization_id: Arc<OrganizationId>,
    reconciliations: Vec<Arc<Entity>>,
}

#[derive(Debug, Serialize)]
pub struct RecordMetadata {
    activated_mapping_id: Arc<MappingId>,
    reconciliations_count: ReconciliationCount,
    processor_id: Arc<ProcessorId>,
}

impl OutputStream<RecordBody, RecordMetadata> for Record {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.aggregate_event.id().to_string())
    }

    fn event(&self) -> StreamEvent {
        match self.aggregate_event.as_ref() {
            AggregateEvent::Created(_) => StreamEvent::Created,
            AggregateEvent::Deleted(_) => StreamEvent::Deleted,
        }
    }

    fn body(&self) -> HashMap<u64, RecordBody> {
        let record_body = RecordBody {
            id: self.aggregate_event.id().clone(),
            organization_id: self.organization_id.clone(),
            reconciliations: self.aggregate_event.reconciliations(),
        };

        let mut body = HashMap::new();
        body.insert(1, record_body);
        body
    }

    fn metadata(&self) -> Option<RecordMetadata> {
        Some(RecordMetadata {
            activated_mapping_id: self.mapping_id.clone(),
            reconciliations_count: self.reconciliations_count.clone(),
            processor_id: self.processor_id.clone(),
        })
    }
}
