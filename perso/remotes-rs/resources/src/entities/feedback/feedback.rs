use super::{FeedbackId, FeedbackMessage};
use crate::entities::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Feedback {
    id: FeedbackId,
    user_id: UserId,
    message: FeedbackMessage,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
