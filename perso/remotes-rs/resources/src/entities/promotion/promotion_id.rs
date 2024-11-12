use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct PromotionId(Uuid);

impl PromotionId {
    pub fn new() -> Self {
        PromotionId(Uuid::new_v4())
    }
}
