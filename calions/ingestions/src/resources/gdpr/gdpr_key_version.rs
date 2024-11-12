use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Debug, From, Deserialize, Serialize)]
pub struct GdprKeyVersion(u32);

impl GdprKeyVersion {
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}
