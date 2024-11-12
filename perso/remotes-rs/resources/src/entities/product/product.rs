use super::{ProductDescription, ProductId, ProductName, ProductPrice};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: ProductDescription,
    price: ProductPrice,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
