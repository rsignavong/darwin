use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub struct MappingField(String);
