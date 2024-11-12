use derive_more::Deref;
use serde::Deserialize;

#[derive(Debug, Deref, Deserialize)]
pub struct S3RegionName(String);
