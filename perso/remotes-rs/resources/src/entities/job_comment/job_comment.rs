use super::{JobCommentId, JobCommentMessage};
use crate::entities::{JobId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JobComment {
    id: JobCommentId,
    job_id: JobId,
    user_id: UserId,
    message: JobCommentMessage,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
