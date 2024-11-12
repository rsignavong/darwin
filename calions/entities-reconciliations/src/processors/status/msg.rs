use crate::processors::StatusProcessorSender;
use crate::resources::{MappingId, ReconciliationCount};
use crate::resources::{StatusReconciliation, StatusState};
use data_stream::stream::InputStream;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum StatusMsg {
    Counts(ReconciliationCount),
    MappingId(Arc<MappingId>, StatusProcessorSender),
    Reconciliations(StatusReconciliation),
    Stream,
    State(StatusState, StatusProcessorSender),
}

impl InputStream for StatusMsg {}
