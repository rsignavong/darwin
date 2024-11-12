use crate::resources::AggregateEvent;
use crate::resources::ReconciliationCount;
use crate::resources::{MappingId, OrganizationId, ProcessorId};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct Record {
    pub aggregate_event: Arc<AggregateEvent>,
    pub mapping_id: Arc<MappingId>,
    pub processor_id: Arc<ProcessorId>,
    pub organization_id: Arc<OrganizationId>,
    pub reconciliations_count: ReconciliationCount,
}
