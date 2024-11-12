use derive_more::From;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, From, Serialize)]
pub struct CompanyId(Uuid);

impl CompanyId {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        CompanyId(Uuid::new_v4())
    }
}
