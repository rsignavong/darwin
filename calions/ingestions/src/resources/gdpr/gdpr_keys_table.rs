use derive_more::Deref;
use serde::Deserialize;

#[derive(Deref, Deserialize)]
pub struct GdprKeysTable(String);
