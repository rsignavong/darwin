use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct JobCommentId(Uuid);

impl JobCommentId {
    pub fn new() -> Self {
        JobCommentId(Uuid::new_v4())
    }
}
