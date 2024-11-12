use aes_gcm::Error as AesGcmError;
use base64::DecodeError;
use std::array::TryFromSliceError;
use std::convert::Infallible;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GdprError {
    #[error("Unable to decode gdpr key")]
    GdprKeyBase64Decode { source: DecodeError },
    #[error("Unable to convert key to bytes")]
    GdprKeyBytesKeyExtraction { source: TryFromSliceError },
    #[error("Unable to convert nonce to bytes")]
    GdprKeyBytesNonceExtraction { source: TryFromSliceError },
    #[error("Wrong key size")]
    GdprKeyWrongSize,
    #[error("Unable to convert bytes to salt")]
    GdprSaltBytesExtraction { source: TryFromSliceError },
    #[error("Unable to extract salt")]
    GdprSaltWrongSize,
    #[error("Unable to decrypt value: {0}")]
    GdprValueAes256GcmDecrypt(AesGcmError),
    #[error("Unable to encrypt value: {0}")]
    GdprValueAes256GcmEncrypt(AesGcmError),
    #[error("Unable to decode gdpr value")]
    GdprValueBase64Decode { source: DecodeError },
    #[error("Unable to convert bytes to gdpr value")]
    GdprValueBytesCipherTextInto { source: Infallible },
    #[error("Unable to convert bytes to utf8 string")]
    GdprValueIntoUtf8 { source: Utf8Error },
    #[error("Unable to extract gdpr value")]
    GdprValueWrongSize,
}
