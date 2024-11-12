use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, From, Serialize)]
pub struct JobCategoryTitle(String);
