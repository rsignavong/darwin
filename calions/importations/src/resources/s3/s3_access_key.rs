use derive_more::Deref;
use serde::Deserialize;

#[derive(Clone, Debug, Deref, Deserialize)]
pub struct S3AccessKey(String);
