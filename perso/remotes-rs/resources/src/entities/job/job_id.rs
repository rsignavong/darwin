use derive_more::From;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, From, Serialize)]
pub struct JobId(Uuid);

impl JobId {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        JobId(Uuid::new_v4())
    }
}
