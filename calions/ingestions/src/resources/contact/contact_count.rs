use super::ContactError;
use derive_more::{Display, From};
use serde::Serialize;
use std::convert::TryFrom;
use tokio_postgres::Row;

#[derive(Clone, Debug, Default, Display, From, Serialize)]
pub struct ContactCount(u64);

impl TryFrom<Row> for ContactCount {
    type Error = ContactError;

    fn try_from(r: Row) -> Result<Self, Self::Error> {
        let count: i64 = r
            .try_get(0)
            .map_err(|source| ContactError::ContactCountFromRow { source })?;

        Ok(ContactCount(count as u64))
    }
}

impl ContactCount {
    pub fn dec(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn inc(&mut self) {
        self.0 += 1;
    }
}
