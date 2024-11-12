use crate::resources::ReconciliationSaga;
use data_stream::stream::{OutputStream, OutputStreamError, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ReconciliationSagaMetadata;

pub type ReconciliationSagaBody = ReconciliationSaga;

impl OutputStream<ReconciliationSagaBody, ReconciliationSagaMetadata> for ReconciliationSaga {
    fn key(&self) -> Result<String, OutputStreamError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, ReconciliationSagaBody> {
        let mut body = HashMap::new();
        body.insert(1, self.clone());
        body
    }

    fn metadata(&self) -> Option<ReconciliationSagaMetadata> {
        None
    }
}
