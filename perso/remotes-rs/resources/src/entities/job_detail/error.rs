use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobDetailError {
    #[error("InvalidApplyEmail {0}")]
    InvalidApplyEmail(String),
    #[error("InvalidPositionLength {0}")]
    InvalidPositionLength(String),
}
