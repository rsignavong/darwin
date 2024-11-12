use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecordSource {
    #[serde(rename = "type")]
    type_: String,
    data: Value,
}
