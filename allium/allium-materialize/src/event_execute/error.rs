use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventExecuteError {
    #[error("from Json")]
    Json(#[from] JsonError),
    #[error("EventExecute")]
    EventExecute(String),
}
