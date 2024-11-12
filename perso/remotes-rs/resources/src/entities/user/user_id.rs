use super::UserError;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deref, Deserialize, From, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    #[cfg(feature = "backend")]
    pub fn new() -> Result<Self, UserError> {
        Ok(UserId(Uuid::new_v4()))
    }
}
