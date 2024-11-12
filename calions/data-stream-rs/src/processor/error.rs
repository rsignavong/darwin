use anyhow::Error as AnyError;
use rayon::ThreadPoolBuildError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("ProcessorConfig")]
    ProcessorConfig(#[from] ProcessorConfigError),
    #[error("ProcessorCreated")]
    ProcessorCreated(#[from] ProcessorCreatedError),
    #[error("ProcessorCustom")]
    ProcessorCustom(#[from] ProcessorCustomError),
    #[error("ProcessorDeleted")]
    ProcessorDeleted(#[from] ProcessorDeletedError),
    #[error("ProcessorEvent")]
    ProcessorEvent(#[from] ProcessorEventError),
    #[error("ProcessorRead")]
    ProcessorRead(#[from] ProcessorReadError),
    #[error("ProcessorStreamNext")]
    ProcessorStreamNext,
    #[error("ProcessorStreamNextInfalliblre")]
    ProcessorStreamNextInfallible,
    #[error("ProcessorSpawnZeroThread")]
    ProcessorSpawnZeroThread,
    #[error("ProcessorThreadPoolBuild")]
    ProcessorThreadPoolBuild(#[from] ThreadPoolBuildError),
    #[error("ProcessorUpdated")]
    ProcessorUpdated(#[from] ProcessorUpdatedError),
}

#[derive(Debug, Error)]
pub enum ProcessorConfigError {
    #[error("ProcessorConfigAny")]
    ProcessorConfigAny(#[from] AnyError),
    #[error("ProcessorConfigBuilder")]
    ProcessorConfigBuilder { source: AnyError },
}

#[derive(Debug, Error)]
pub enum ProcessorCreatedError {
    #[error("ProcessorCreatedAny")]
    ProcessorCreatedAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum ProcessorCustomError {
    #[error("ProcessorCustomAny")]
    ProcessorCustomAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum ProcessorDeletedError {
    #[error("ProcessorDeletedAny")]
    ProcessorDeletedAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum ProcessorEventError {
    #[error("ProcessorEventAny")]
    ProcessorEventAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum ProcessorReadError {
    #[error("ProcessorReadAny")]
    ProcessorReadAny(#[from] AnyError),
}

#[derive(Debug, Error)]
pub enum ProcessorUpdatedError {
    #[error("ProcessorUpdatedAny")]
    ProcessorUpdatedAny(#[from] AnyError),
}
