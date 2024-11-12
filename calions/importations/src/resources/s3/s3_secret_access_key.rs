use derive_more::Deref;
use serde::Deserialize;

#[derive(Clone, Deref, Deserialize)]
pub struct S3SecretAccessKey(String);
