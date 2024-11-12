use crate::resources::{MappingId, Record, RecordData, RecordProfile, RecordSource};
use crate::resources::{ProfileCount, ProfileId, ReconciliationCount, TransactionId};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct RecordBody {
    id: Arc<RecordProfile>,
    record: Arc<RecordData>,
}

#[derive(Debug, Serialize)]
pub struct RecordMetadata {
    activated_mapping_id: Arc<MappingId>,
    merges: Arc<Vec<ProfileId>>,
    profiles_count: Arc<ProfileCount>,
    reconciliations_count: Arc<ReconciliationCount>,
    source: Arc<RecordSource>,
    transaction_id: Arc<TransactionId>,
}

impl OutputStream<RecordBody, RecordMetadata> for Record {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.profile.to_string())
    }

    fn event(&self) -> StreamEvent {
        match *self.profile {
            RecordProfile::Created(_) => StreamEvent::Created,
            RecordProfile::Updated(_) => StreamEvent::Updated,
        }
    }

    fn body(&self) -> HashMap<u64, RecordBody> {
        let record_body = RecordBody {
            id: self.profile.clone(),
            record: self.data.clone(),
        };

        let mut body = HashMap::new();
        body.insert(1, record_body);
        body
    }

    fn metadata(&self) -> Option<RecordMetadata> {
        Some(RecordMetadata {
            activated_mapping_id: self.mapping_id.clone(),
            merges: self.merges.clone(),
            profiles_count: self.profiles_count.clone(),
            reconciliations_count: self.reconciliations_count.clone(),
            source: self.source.clone(),
            transaction_id: self.transaction_id.clone(),
        })
    }
}
