use crate::resources::ResourcesError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvSeparator(String);

impl CsvSeparator {
    pub fn as_byte(&self) -> Result<u8, ResourcesError> {
        Ok(*self
            .0
            .as_bytes()
            .first()
            .ok_or_else(|| ResourcesError::CsvSeparatorByte(self.0.to_owned()))?)
    }
}
