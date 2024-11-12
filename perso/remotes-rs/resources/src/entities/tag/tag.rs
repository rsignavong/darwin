use super::{TagId, TagTitle};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tag {
    id: TagId,
    title: TagTitle,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
