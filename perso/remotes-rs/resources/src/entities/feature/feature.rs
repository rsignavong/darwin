use super::{FeatureDescription, FeatureId, FeatureName};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Feature {
    id: FeatureId,
    name: FeatureName,
    description: FeatureDescription,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
