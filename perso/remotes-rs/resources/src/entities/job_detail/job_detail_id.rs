use derive_more::From;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, From, Serialize)]
pub struct JobDetailId(Uuid);

impl JobDetailId {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        JobDetailId(Uuid::new_v4())
    }
}
