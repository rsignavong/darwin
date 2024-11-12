use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct PurchaseOrderId(Uuid);

impl PurchaseOrderId {
    pub fn new() -> Self {
        PurchaseOrderId(Uuid::new_v4())
    }
}
