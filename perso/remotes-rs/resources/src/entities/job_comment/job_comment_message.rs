use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JobCommentMessage(String);
