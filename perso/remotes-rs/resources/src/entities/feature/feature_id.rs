use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct FeatureId(Uuid);

impl FeatureId {
    pub fn new() -> Self {
        FeatureId(Uuid::new_v4())
    }
}
