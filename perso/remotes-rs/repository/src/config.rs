use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryConfig {
    pub database: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub pool_size: usize,
    pub port: u16,
}
