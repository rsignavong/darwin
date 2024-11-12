use super::AggregateId;
use crate::resources::RecordReconciliations;
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct AggregateEntry {
    id: Arc<AggregateId>,
    reconciliations: RecordReconciliations,
}

impl AggregateEntry {
    pub fn id(&self) -> &Arc<AggregateId> {
        &self.id
    }

    pub fn reconciliations(&self) -> &RecordReconciliations {
        &self.reconciliations
    }
}
