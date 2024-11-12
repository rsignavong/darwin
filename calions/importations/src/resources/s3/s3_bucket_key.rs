use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
pub struct S3BucketKey(String);
