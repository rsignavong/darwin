use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnonymizationValidated(bool);

impl AnonymizationValidated {
    pub fn is_validated(&self) -> bool {
        self.0
    }
}
