use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PromotionDescription(String);
