use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Deref, Display, Serialize)]
pub struct ProcessorTopic(String);
