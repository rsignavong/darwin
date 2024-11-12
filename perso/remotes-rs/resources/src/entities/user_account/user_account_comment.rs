use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, new)]
pub struct UserAccountComment(String);
