use super::error::GdprError;
use rand::{thread_rng, Rng};
use std::convert::{TryFrom, TryInto};

pub const SALT_LENGTH: usize = 16;

pub struct GdprSalt([u8; SALT_LENGTH]);

impl TryFrom<&[u8]> for GdprSalt {
    type Error = GdprError;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.len() < SALT_LENGTH {
            return Err(GdprError::GdprSaltWrongSize);
        }
        let salt: [u8; SALT_LENGTH] = b[0..SALT_LENGTH]
            .try_into()
            .map_err(|source| GdprError::GdprSaltBytesExtraction { source })?;

        Ok(GdprSalt(salt))
    }
}

impl GdprSalt {
    pub fn gen() -> Self {
        let salt = thread_rng().gen::<[u8; SALT_LENGTH]>();

        GdprSalt(salt)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
