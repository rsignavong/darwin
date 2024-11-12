use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct EntityRecord(Value);

impl EntityRecord {
    pub fn get(&self, pointer: &str) -> Option<&Value> {
        self.0.pointer(pointer)
    }

    pub fn value(&self) -> &Value {
        &self.0
    }
}
