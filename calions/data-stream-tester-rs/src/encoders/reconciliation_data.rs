use crate::resources::ReconciliationData;
use data_stream::stream::{OutputStream, OutputStreamError, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ReconciliationDataMetadata;

pub type ReconciliationDataBody = ReconciliationData;

impl OutputStream<ReconciliationDataBody, ReconciliationDataMetadata> for ReconciliationData {
    fn key(&self) -> Result<String, OutputStreamError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, ReconciliationDataBody> {
        let mut body = HashMap::new();
        body.insert(1, self.clone());
        body
    }

    fn metadata(&self) -> Option<ReconciliationDataMetadata> {
        None
    }
}
