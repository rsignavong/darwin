use super::{AggregateCount, AggregateCountReconciliations, AggregateSet};
use crate::resources::{RecordReconciliations, ResourcesError};
use ahash::AHashMap;
use derive_new::new;

#[derive(new)]
pub struct AggregateCountMap(#[new(default)] AHashMap<AggregateSet, AggregateCountReconciliations>);

impl AggregateCountMap {
    pub fn add(&mut self, aggregates: AggregateSet, reconciliations: RecordReconciliations) {
        if let Some(count_reconciliations) = self.0.get_mut(&aggregates) {
            count_reconciliations.inc_count();
        } else {
            self.0.insert(
                aggregates,
                AggregateCountReconciliations::new(reconciliations),
            );
        }
    }

    pub fn for_each<F>(self, mut func: F) -> Result<(), ResourcesError>
    where
        F: FnMut(AggregateSet, RecordReconciliations, AggregateCount) -> Result<(), ResourcesError>,
    {
        for (aggregates, cr) in self.0.into_iter() {
            func(aggregates, cr.reconciliations().clone(), cr.count().clone())?;
        }

        Ok(())
    }
}
