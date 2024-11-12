use derive_more::From;
use serde::Serialize;

#[derive(Clone, Debug, Default, From, Serialize)]
pub struct ReconciliationCount(u64);

impl ReconciliationCount {
    pub fn set(&mut self, count: u64) {
        self.0 = count;
    }
}
