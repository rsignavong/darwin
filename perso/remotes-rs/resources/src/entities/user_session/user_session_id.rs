use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSessionId(Uuid);

impl UserSessionId {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        UserSessionId(Uuid::new_v4())
    }
}
