use super::{UserAccountComment, UserAccountId, UserAccountStatus};
use crate::entities::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserAccount {
    id: UserAccountId,
    user_id: UserId,
    status: UserAccountStatus,
    comment: Option<UserAccountComment>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
