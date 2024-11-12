use super::Index;
use ulid::Ulid;

#[derive(Debug, Fail)]
pub enum ResourceDataError {
    #[fail(display = "Resource data not found: {}", _0)]
    DataNotFound(String),
}

pub type Value = Ulid;

#[derive(Clone, Debug, Default)]
pub struct Key(String);

impl Key {
    #[inline]
    pub fn new(data: &str) -> Self {
        Key(data.to_owned)
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn index(&self) -> Option<Index> {
        self.0.chars().next()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Default)]
pub struct ResourceData(Key, Option<Value>);

impl ResourceData {
    #[inline]
    pub fn new(key: Key, value: Option<Value>) -> Self {
        ResourceData(key, value)
    }

    #[inline]
    pub fn key(&self) -> Key {
        self.0
    }
}
