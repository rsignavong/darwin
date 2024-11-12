use super::AggregateEvent;
use crate::resources::{MappingId, ProcessorId, ReconciliationCount, Record};
use crate::Settings;
use derive_new::new;
use std::sync::Arc;
use std::vec::IntoIter;

#[derive(Debug, new)]
pub struct AggregateEventList(#[new(default)] Vec<AggregateEvent>);

impl AggregateEventList {
    pub fn add_event(&mut self, event: AggregateEvent) {
        self.0.push(event);
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.into_iter());
    }

    fn into_iter(self) -> IntoIter<AggregateEvent> {
        self.0.into_iter()
    }

    pub fn map_records(
        self,
        mapping_id: &Arc<MappingId>,
        processor_id: &Arc<ProcessorId>,
        count: &ReconciliationCount,
    ) -> Vec<Record> {
        self.0
            .into_iter()
            .map(|event| {
                Record::new(
                    Arc::new(event),
                    mapping_id.clone(),
                    processor_id.clone(),
                    Settings::get().organization_id.clone(),
                    count.clone(),
                )
            })
            .collect()
    }
}
