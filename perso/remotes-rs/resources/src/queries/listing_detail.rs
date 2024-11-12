use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct ListingDetail {
    detail: String,
}
