use thiserror::Error;

#[derive(Debug, Error)]
pub enum YamlError {
    #[error("from Json")]
    Json(#[from] serde_json::Error),
    #[error("from Yaml")]
    Yaml(#[from] serde_yaml::Error),
}
