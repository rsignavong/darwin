use derive_more::Deref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Deserialize, Serialize)]
pub struct S3BucketName(String);
