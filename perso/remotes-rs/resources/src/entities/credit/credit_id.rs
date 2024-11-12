use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreditId(Uuid);

impl CreditId {
    pub fn new() -> Self {
        CreditId(Uuid::new_v4())
    }
}
