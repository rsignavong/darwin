use derive_more::{Deref, Display};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deref, Deserialize, Display, Eq, Hash, PartialEq, Serialize, new)]
pub struct MappingField(String);
