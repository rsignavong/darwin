use super::{RecordData, RecordProfile, RecordSource};
use crate::resources::TransactionId;
use crate::resources::{MappingId, ProcessorId, ReconciliationCount};
use crate::resources::{ProfileCount, ProfileId};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct Record {
    pub data: Arc<RecordData>,
    pub mapping_id: Arc<MappingId>,
    pub merges: Arc<Vec<ProfileId>>,
    pub processor_id: Arc<ProcessorId>,
    pub profile: Arc<RecordProfile>,
    pub profiles_count: Arc<ProfileCount>,
    pub reconciliations_count: Arc<ReconciliationCount>,
    pub source: Arc<RecordSource>,
    pub transaction_id: Arc<TransactionId>,
}
