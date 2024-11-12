use super::AggregateSet;
use crate::resources::{NodeEntityList, RecordReconciliations};
use derive_new::new;

#[derive(new)]
pub struct AggregateRecord {
    aggregates: AggregateSet,
    reconciliations: RecordReconciliations,
}

impl From<NodeEntityList> for AggregateRecord {
    fn from(node_entities: NodeEntityList) -> Self {
        let size = node_entities.len();
        let mut aggregates = AggregateSet::with_capacity(size);
        let mut reconciliations = RecordReconciliations::with_capacity(size);

        node_entities.extract(&mut aggregates, &mut reconciliations);

        AggregateRecord {
            aggregates,
            reconciliations,
        }
    }
}

impl AggregateRecord {
    pub fn aggregates(&self) -> &AggregateSet {
        &self.aggregates
    }

    pub fn reconciliations(&self) -> &RecordReconciliations {
        &self.reconciliations
    }
}
