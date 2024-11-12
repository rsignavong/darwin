use derive_more::Deref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MappingField(String);
