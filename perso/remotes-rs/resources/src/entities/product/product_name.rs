use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductName(String);
