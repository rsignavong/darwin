use super::{UserSessionCode, UserSessionId, UserSessionToken};
use crate::entities::UserAccount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserSession {
    id: UserSessionId,
    user_account_id: UserAccountId,
    token: UserSessionToken,
    code: UserSessionCode,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
