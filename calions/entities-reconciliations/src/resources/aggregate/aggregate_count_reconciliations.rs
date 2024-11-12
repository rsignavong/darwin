use super::AggregateCount;
use crate::resources::RecordReconciliations;

pub struct AggregateCountReconciliations {
    count: AggregateCount,
    reconciliations: RecordReconciliations,
}

impl AggregateCountReconciliations {
    pub fn count(&self) -> &AggregateCount {
        &self.count
    }

    pub fn inc_count(&mut self) {
        self.count.inc();
    }

    pub fn new(reconciliations: RecordReconciliations) -> Self {
        AggregateCountReconciliations {
            count: AggregateCount::new(),
            reconciliations,
        }
    }

    pub fn reconciliations(&self) -> &RecordReconciliations {
        &self.reconciliations
    }
}
