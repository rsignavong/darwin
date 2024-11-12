use super::{AggregateCount, AggregateId};
use crate::resources::ResourcesError;
use std::ops::Sub;
use std::sync::Arc;

#[derive(Debug)]
pub struct AggregateRc {
    count: AggregateCount,
    id: Arc<AggregateId>,
}

impl AggregateRc {
    pub fn sub(&mut self, count: AggregateCount) -> bool {
        self.count = self.count.sub(count);
        self.count.is_zero()
    }

    pub fn set_count(&mut self, count: AggregateCount) {
        self.count = count;
    }

    pub fn id(&self) -> &Arc<AggregateId> {
        &self.id
    }

    pub fn new(count: AggregateCount) -> Result<Self, ResourcesError> {
        let id = Arc::new(AggregateId::new()?);

        Ok(AggregateRc { count, id })
    }
}
