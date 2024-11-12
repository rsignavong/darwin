use crate::processors::StatusProcessorSender;
use crate::resources::{ContactCount, MappingId, StatusIngestion, StatusState};
use data_stream::stream::InputStream;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum StatusMsg {
    Count(Arc<ContactCount>),
    Ingestions(StatusIngestion),
    MappingId(Arc<MappingId>, StatusProcessorSender),
    State(StatusState, StatusProcessorSender),
    Stream,
}

impl InputStream for StatusMsg {}
