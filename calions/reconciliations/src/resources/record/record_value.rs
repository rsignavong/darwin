use derive_more::Display;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize, new)]
pub struct RecordValue(String);
