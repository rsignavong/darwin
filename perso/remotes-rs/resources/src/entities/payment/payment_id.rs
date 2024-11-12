use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct PaymentId(Uuid);

impl PaymentId {
    pub fn new() -> Self {
        PaymentId(Uuid::new_v4())
    }
}
