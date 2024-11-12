use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deref, Deserialize, From, Serialize)]
pub struct JobCategoryId(Uuid);

impl JobCategoryId {
    pub fn new() -> Self {
        JobCategoryId(Uuid::new_v4())
    }
}
