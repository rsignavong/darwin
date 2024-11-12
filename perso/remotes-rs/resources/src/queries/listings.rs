use derive_new::new;
use serde::Serialize;

#[derive(Debug, Serialize, new)]
pub struct Listings {
    listings: Vec<String>,
    current_page: usize,
    last_page: usize,
}
