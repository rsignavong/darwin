use super::Cookie;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub domains: Vec<String>,
    pub cookie: Cookie,
    pub listen: String,
}
