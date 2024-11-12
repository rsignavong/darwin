use derive_more::From;
use serde::Serialize;

#[derive(Clone, Debug, Default, From, Serialize)]
pub struct ImportationProgression(f64);
