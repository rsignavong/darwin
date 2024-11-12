use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Display, Eq, PartialEq, Serialize)]
pub struct MappingId(String);
