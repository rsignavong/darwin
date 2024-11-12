use crate::resources::ActivatedMapping;
use data_stream::stream::{OutputStream, OutputStreamError, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ActivatedMappingMetadata;

pub type ActivatedMappingBody = ActivatedMapping;

impl OutputStream<ActivatedMappingBody, ActivatedMappingMetadata> for ActivatedMapping {
    fn key(&self) -> Result<String, OutputStreamError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        StreamEvent::Created
    }

    fn body(&self) -> HashMap<u64, ActivatedMappingBody> {
        let mut body = HashMap::new();
        body.insert(1, self.clone());
        body
    }

    fn metadata(&self) -> Option<ActivatedMappingMetadata> {
        None
    }
}
