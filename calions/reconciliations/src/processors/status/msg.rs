use crate::processors::StatusProcessorSender;
use crate::resources::{MappingId, ProfileCount, ReconciliationCount};
use crate::resources::{StatusReconciliation, StatusState};
use data_stream::stream::InputStream;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum StatusMsg {
    Counts(Arc<ProfileCount>, Arc<ReconciliationCount>),
    MappingId(Arc<MappingId>, StatusProcessorSender),
    Reconciliations(StatusReconciliation),
    State(StatusState, StatusProcessorSender),
    Stream,
}

impl InputStream for StatusMsg {}
