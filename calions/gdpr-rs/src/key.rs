use super::error::GdprError;
use base64::{decode, encode};
use rand::{thread_rng, Rng};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, Result as FmtResult};

const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

pub struct GdprKey {
    key: [u8; KEY_LENGTH],
    nonce: [u8; NONCE_LENGTH],
}

impl Display for GdprKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = encode(self.as_bytes());
        write!(f, "{}", s)
    }
}

impl TryFrom<&[u8]> for GdprKey {
    type Error = GdprError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != (KEY_LENGTH + NONCE_LENGTH) {
            return Err(GdprError::GdprKeyWrongSize);
        }
        let key: [u8; KEY_LENGTH] = bytes[0..KEY_LENGTH]
            .try_into()
            .map_err(|source| GdprError::GdprKeyBytesKeyExtraction { source })?;
        let nonce: [u8; NONCE_LENGTH] = bytes[KEY_LENGTH..]
            .try_into()
            .map_err(|source| GdprError::GdprKeyBytesNonceExtraction { source })?;

        Ok(GdprKey { key, nonce })
    }
}

impl TryFrom<Vec<u8>> for GdprKey {
    type Error = GdprError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() != (KEY_LENGTH + NONCE_LENGTH) {
            return Err(GdprError::GdprKeyWrongSize);
        }
        let key: [u8; KEY_LENGTH] = bytes[0..KEY_LENGTH]
            .try_into()
            .map_err(|source| GdprError::GdprKeyBytesKeyExtraction { source })?;
        let nonce: [u8; NONCE_LENGTH] = bytes[KEY_LENGTH..]
            .try_into()
            .map_err(|source| GdprError::GdprKeyBytesNonceExtraction { source })?;

        Ok(GdprKey { key, nonce })
    }
}

impl TryFrom<&str> for GdprKey {
    type Error = GdprError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let bytes = decode(s).map_err(|source| GdprError::GdprKeyBase64Decode { source })?;
        GdprKey::try_from(bytes)
    }
}

impl GdprKey {
    pub fn as_bytes(&self) -> Vec<u8> {
        [self.key.as_ref(), self.nonce.as_ref()].concat()
    }

    pub fn gen() -> Result<Self, GdprError> {
        let key = thread_rng().gen::<[u8; KEY_LENGTH]>();
        let nonce = thread_rng().gen::<[u8; NONCE_LENGTH]>();
        Ok(GdprKey { key, nonce })
    }

    pub fn key(&self) -> &[u8; KEY_LENGTH] {
        &self.key
    }
    pub fn nonce(&self) -> &[u8; NONCE_LENGTH] {
        &self.nonce
    }
}
