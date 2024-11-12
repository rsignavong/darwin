use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct TagId(Uuid);

impl TagId {
    pub fn new() -> Self {
        TagId(Uuid::new_v4())
    }
}
