use super::CreditId;
use crate::entities::{Feature, Job, PurchaseOrderId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credit {
    id: CreditId,
    purchase_order_id: PurchaseOrderId,
    feature: Feature,
    job: Job,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
