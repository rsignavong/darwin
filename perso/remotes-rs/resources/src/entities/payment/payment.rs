use super::{PaymentId, PaymentMetadata};
use crate::entities::PurchaseOrderId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payment {
    id: PaymentId,
    purchase_order_id: PurchaseOrderId,
    metadata: PaymentMetadata,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
