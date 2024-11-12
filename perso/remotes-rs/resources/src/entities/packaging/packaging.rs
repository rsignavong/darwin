use crate::entities::{FeatureId, ProductId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Packaging {
    feature_id: FeatureId,
    product_id: ProductId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
