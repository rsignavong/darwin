use derive_more::{Deref, Display};
use serde::Deserialize;

#[derive(Debug, Deserialize, Deref, Display)]
pub struct ProcessorTopic(String);
