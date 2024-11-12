use chrono::{NaiveDateTime, Utc};
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deref, Deserialize, From, Serialize)]
pub struct GdprKeyDateTime(NaiveDateTime);

impl GdprKeyDateTime {
    pub fn new() -> Self {
        let now = Utc::now().naive_utc();
        GdprKeyDateTime(now)
    }
}
