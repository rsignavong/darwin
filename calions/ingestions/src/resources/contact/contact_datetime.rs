use chrono::{NaiveDateTime, Utc};
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deref, Deserialize, From, Serialize)]
pub struct ContactDateTime(NaiveDateTime);

impl ContactDateTime {
    pub fn new() -> Self {
        let now = Utc::now().naive_utc();
        ContactDateTime(now)
    }
}
