use super::{PurchaseOrderData, PurchaseOrderId};
use crate::entities::{CompanyId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PurchaseOrder {
    id: PurchaseOrderId,
    company_id: CompanyId,
    user_id: UserId,
    data: PurchaseOrderData,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
