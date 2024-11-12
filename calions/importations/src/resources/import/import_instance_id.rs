use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Display, Serialize)]
pub struct ImportInstanceId(String);
