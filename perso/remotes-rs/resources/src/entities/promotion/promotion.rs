use super::{PromotionDate, PromotionDescription, PromotionId, PromotionName, PromotionPrice};
use crate::entities::ProductId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Promotion {
    id: PromotionId,
    product_id: ProductId,
    name: PromotionName,
    description: PromotionDescription,
    begin: PromotionDate,
    end: PromotionDate,
    price: PromotionPrice,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
