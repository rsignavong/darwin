use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("InvalidEmail {0}")]
    InvalidEmail(String),
}
