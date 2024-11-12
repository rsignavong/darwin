use crate::decoders::ImportInstanceBody;
use derive_more::{Add, Deref, From};
use serde::{Deserialize, Serialize};
use std::ops::Add as _;

#[derive(Add, Clone, Debug, Deref, Deserialize, Eq, From, PartialEq, Serialize)]
pub struct CsvLineCount(u64);

impl From<&ImportInstanceBody> for CsvLineCount {
    fn from(import_instance_body: &ImportInstanceBody) -> Self {
        CsvLineCount(
            *import_instance_body
                .config
                .files
                .iter()
                .fold(0.into(), |acc: Self, file| acc.add(file.lines.clone())),
        )
    }
}
