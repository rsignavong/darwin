use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Node {
    pub id: String,
    pub next: Vec<String>,
}
