use super::{UserEmail, UserId};
use crate::entities::CompanyId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: UserId,
    email: UserEmail,
    company_id: Option<CompanyId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
