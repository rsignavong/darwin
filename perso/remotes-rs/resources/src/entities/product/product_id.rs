use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new() -> Self {
        ProductId(Uuid::new_v4())
    }
}
