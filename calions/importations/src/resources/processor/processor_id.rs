use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Display, Eq, PartialEq, Serialize)]
pub struct ProcessorId(String);
