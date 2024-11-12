use super::IngestionDataMapping;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum IngestionData {
    Mapping(IngestionDataMapping),
}
