use derive_more::{Deref, Display};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Deref, Display)]
pub struct ProcessorTopic(String);
