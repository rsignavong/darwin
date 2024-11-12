use derive_more::From;
use serde::Serialize;

#[derive(Debug, Default, From, Serialize)]
pub struct ProfileCount(u64);
