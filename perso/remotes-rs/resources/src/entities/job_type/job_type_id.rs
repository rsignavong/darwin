use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deref, Deserialize, From, Serialize)]
pub struct JobTypeId(Uuid);

impl JobTypeId {
    pub fn new() -> Self {
        JobTypeId(Uuid::new_v4())
    }
}
