use super::storage::ManifestError;
use super::storage::ManifestWriterError;
use super::storage::ResourceDataError;
use super::storage::ResourceError;
use super::storage::ResourceWriterError;
use super::storage::ScannerError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum TeslaError {
    Manifest(ManifestError),
    ManifestWriter(ManifestWriterError),
    Resource(ResourceError),
    ResourceData(ResourceDataError),
    ResourceWriter(ResourceWriterError),
    Scanner(ScannerError),
}

impl Display for TeslaError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            TeslaError::Manifest(ref e) => write!(f, "Tesla::Manifest: {}", e),
            TeslaError::ManifestWriter(ref e) => write!(f, "Tesla::ManifestWriter: {}", e),
            TeslaError::Resource(ref e) => write!(f, "Tesla::Resource: {}", e),
            TeslaError::ResourceData(ref e) => write!(f, "Tesla::ResourceData: {}", e),
            TeslaError::ResourceWriter(ref e) => write!(f, "Tesla::ResourceWriter: {}", e),
            TeslaError::Scanner(ref e) => write!(f, "Tesla::Scanner: {}", e),
        }
    }
}

impl From<ManifestError> for TeslaError {
    fn from(err: ManifestError) -> Self {
        TeslaError::Manifest(err)
    }
}

impl From<ManifestWriterError> for TeslaError {
    fn from(err: ManifestWriterError) -> Self {
        TeslaError::ManifestWriter(err)
    }
}

impl From<ResourceError> for TeslaError {
    fn from(err: ResourceError) -> Self {
        TeslaError::Resource(err)
    }
}

impl From<ResourceDataError> for TeslaError {
    fn from(err: ResourceDataError) -> Self {
        TeslaError::ResourceData(err)
    }
}

impl From<ResourceWriterError> for TeslaError {
    fn from(err: ResourceWriterError) -> Self {
        TeslaError::ResourceWriter(err)
    }
}

impl From<ScannerError> for TeslaError {
    fn from(err: ScannerError) -> Self {
        TeslaError::Scanner(err)
    }
}
