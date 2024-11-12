use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAccountId(Uuid);

impl UserAccountId {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        UserAccountId(Uuid::new_v4())
    }
}
