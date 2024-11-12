use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PromotionDate(DateTime<Utc>);

impl PromotionDate {
    pub fn new() -> Self {
        PromotionDate(Utc::now())
    }
}
