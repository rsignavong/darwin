use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductPrice(u32);
