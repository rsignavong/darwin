use crate::resources::RawRecord;
use data_stream::stream::{OutputStream, OutputStreamError, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct RawRecordMetadata;

pub type RawRecordBody = RawRecord;

impl OutputStream<RawRecordBody, RawRecordMetadata> for RawRecord {
    fn key(&self) -> Result<String, OutputStreamError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, RawRecordBody> {
        let mut body = HashMap::new();
        body.insert(1, self.clone());
        body
    }

    fn metadata(&self) -> Option<RawRecordMetadata> {
        None
    }
}
