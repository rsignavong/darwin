use derive_more::Deref;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Deref)]
pub struct ProcessorTopic(String);
