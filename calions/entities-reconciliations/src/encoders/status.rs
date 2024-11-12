use crate::resources::Status;
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct StatusMetadata;

pub type StatusBody = Status;

impl OutputStream<StatusBody, StatusMetadata> for Status {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.processor_id.to_string())
    }

    fn event(&self) -> StreamEvent {
        match self.mapping_id {
            Some(_) => StreamEvent::Updated,
            None => StreamEvent::Created,
        }
    }

    fn body(&self) -> HashMap<u64, StatusBody> {
        let mut body = HashMap::new();
        body.insert(1, self.clone());
        body
    }
}
