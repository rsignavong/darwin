use super::GdprError;
use derivative::Derivative;
use gdpr::{serialize_gdpr_key, GdprKey};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

#[derive(Derivative, Serialize)]
#[derivative(Debug)]
pub struct GdprKeyKey(
    #[derivative(Debug = "ignore")]
    #[serde(serialize_with = "serialize_gdpr_key")]
    GdprKey,
);

impl TryFrom<&[u8]> for GdprKeyKey {
    type Error = GdprError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(GdprKeyKey(bytes.try_into()?))
    }
}

impl GdprKeyKey {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }

    pub fn inner(&self) -> &GdprKey {
        &self.0
    }

    pub fn new() -> Result<Self, GdprError> {
        Ok(GdprKeyKey(GdprKey::gen()?))
    }
}
