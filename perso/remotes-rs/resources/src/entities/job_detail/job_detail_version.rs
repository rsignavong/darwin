use derive_more::{Deref, From};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Deserialize, From, Serialize, new)]
pub struct JobDetailVersion(#[new(value = "1")] i16);

impl JobDetailVersion {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}
