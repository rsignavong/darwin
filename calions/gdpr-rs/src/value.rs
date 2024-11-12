use super::error::GdprError;
use super::key::GdprKey;
use super::salt::{GdprSalt, SALT_LENGTH};
use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;
use base64::{decode, encode};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::from_utf8;

pub struct GdprValue {
    salt: GdprSalt,
    ciphertext: Vec<u8>,
}

impl Display for GdprValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = encode([self.salt.as_slice(), &self.ciphertext].concat());
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for GdprValue {
    type Error = GdprError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let bytes = decode(s).map_err(|source| GdprError::GdprValueBase64Decode { source })?;
        let salt = GdprSalt::try_from(bytes.as_ref())?;
        if bytes.len() <= SALT_LENGTH {
            return Err(GdprError::GdprValueWrongSize);
        }
        let ciphertext: Vec<u8> = bytes[SALT_LENGTH..]
            .try_into()
            .map_err(|source| GdprError::GdprValueBytesCipherTextInto { source })?;

        Ok(GdprValue { salt, ciphertext })
    }
}

impl GdprValue {
    pub fn decrypt(&self, gdpr_key: &GdprKey) -> Result<String, GdprError> {
        let key = GenericArray::from_slice(gdpr_key.key());
        let nonce = GenericArray::from_slice(gdpr_key.nonce());

        let cipher = Aes256Gcm::new(key);
        let decrypted = cipher
            .decrypt(nonce, self.ciphertext.as_ref())
            .map_err(GdprError::GdprValueAes256GcmDecrypt)?;
        let text =
            from_utf8(&decrypted).map_err(|source| GdprError::GdprValueIntoUtf8 { source })?;

        Ok(text.to_owned())
    }

    pub fn encrypt(value: &str) -> Result<(GdprKey, Self), GdprError> {
        let gdpr_key = GdprKey::gen()?;
        let gdpr_value = Self::encrypt_with_key(value, &gdpr_key)?;
        Ok((gdpr_key, gdpr_value))
    }

    pub fn encrypt_with_key(value: &str, gdpr_key: &GdprKey) -> Result<Self, GdprError> {
        let key = GenericArray::from_slice(gdpr_key.key());
        let nonce = GenericArray::from_slice(gdpr_key.nonce());
        let salt = GdprSalt::gen();

        let cipher = Aes256Gcm::new(key);
        let ciphertext = cipher
            .encrypt(nonce, value.as_bytes())
            .map_err(GdprError::GdprValueAes256GcmEncrypt)?;

        Ok(GdprValue { salt, ciphertext })
    }
}
