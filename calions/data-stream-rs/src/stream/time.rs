use crate::consumer::ConsumerError;
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
#[serde(try_from = "i64", into = "i64")]
pub struct ProcessingTime(#[new(value = "Utc::now()")] DateTime<Utc>);

impl Into<i64> for ProcessingTime {
    fn into(self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl TryFrom<i64> for ProcessingTime {
    type Error = ConsumerError;

    fn try_from(ms: i64) -> Result<Self, Self::Error> {
        match Utc.timestamp_millis_opt(ms) {
            LocalResult::Single(dt) => Ok(ProcessingTime(dt)),
            _ => Err(ConsumerError::ConsumerDeserializeProcessingTime(ms)),
        }
    }
}

impl ProcessingTime {
    pub fn with_datetime(dt: DateTime<Utc>) -> Self {
        ProcessingTime(dt)
    }
}
