use anyhow::Error as AnyError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputStreamError {
    #[error("OutputStreamEvent")]
    OutputStreamEvent(#[from] OutputStreamEventError),
    #[error("OutputStreamFormat")]
    OutputStreamFormat(#[from] OutputStreamFormatError),
    #[error("OutputStreamKey")]
    OutputStreamKey(#[from] OutputStreamKeyError),
    #[error("OutputStreamNew")]
    OutputStreamNew(#[from] OutputStreamNewError),
    #[error("OutputStreamOld")]
    OutputStreamOld(#[from] OutputStreamOldError),
    #[error("OutputStreamSchema")]
    OutputStreamSchema(#[from] OutputStreamSchemaError),
    #[error("OutputStreamSource")]
    OutputStreamSource(#[from] OutputStreamSourceError),
    #[error("OutputStreamTime")]
    OutputStreamTime(#[from] OutputStreamTimeError),
    #[error("OutputStreamToJson")]
    OutputStreamToJson(#[from] SerdeJsonError),
}

#[derive(Debug, Error)]
pub enum OutputStreamEventError {
    #[error("OutputStreamEventAny")]
    OutputStreamEventAny(#[from] AnyError),
    #[error("OutputStreamEventNew")]
    OutputStreamEventNew(#[from] OutputStreamNewError),
    #[error("OutputStreamEventOld")]
    OutputStreamEventOld(#[from] OutputStreamOldError),
}

#[derive(Debug, Error)]
pub enum OutputStreamFormatError {
    #[error("OutputStreamFormatAny")]
    OutputStreamFormatAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamKeyError {
    #[error("OutputStreamKeyAny")]
    OutputStreamKeyAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamNewError {
    #[error("OutputStreamNewAny")]
    OutputStreamNewAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamOldError {
    #[error("OutputStreamOldAny")]
    OutputStreamOldAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamSchemaError {
    #[error("OutputStreamSchemaAny")]
    OutputStreamSchemaAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamSourceError {
    #[error("OutputStreamSourceAny")]
    OutputStreamSourceAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum OutputStreamTimeError {
    #[error("OutputStreamTimeAny")]
    OutputStreamTimeAny(#[from] AnyError),
}
