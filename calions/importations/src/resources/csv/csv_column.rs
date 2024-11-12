use derive_more::Display;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Display)]
pub struct CsvColumn(u64);

impl CsvColumn {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

