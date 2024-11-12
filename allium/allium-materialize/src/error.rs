use crate::event_execute::EventExecuteError;
use crate::yaml::YamlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlliumMaterializeError {
    #[error("Tree error")]
    EventExecute(#[from] EventExecuteError),
    #[error("Yaml serialization error")]
    Yaml(#[from] YamlError),
}
