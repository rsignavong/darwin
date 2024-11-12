use derivative::Derivative;
use serde::Serialize;

#[derive(Clone, Debug, Derivative, Serialize)]
#[serde(rename_all = "kebab-case")]
#[derivative(Default)]
pub enum StatusImportation {
    #[derivative(Default)]
    Ready,
    ImportingFiles,
    ImportationCompleted,
}