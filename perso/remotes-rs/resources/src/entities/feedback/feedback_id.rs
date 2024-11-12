use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct FeedbackId(Uuid);

impl FeedbackId {
    pub fn new() -> Self {
        FeedbackId(Uuid::new_v4())
    }
}
