use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, From, PartialEq, PartialOrd, Serialize)]
pub struct ReconciliationCount(u64);

impl ReconciliationCount {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}
