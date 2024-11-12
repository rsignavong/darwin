use super::PromotionDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PromotionPeriod {
    begin: PromotionDate,
    end: PromotionDate,
}
