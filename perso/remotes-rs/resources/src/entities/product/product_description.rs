use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductDescription(String);
