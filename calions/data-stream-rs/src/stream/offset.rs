use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Earlier,
    Latest,
    Above,
}

#[derive(
    Clone, Debug, Deref, Deserialize, Display, Eq, From, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Offset(i64);

impl Offset {
    pub fn is_invalid(&self) -> bool {
        self.0 < 0
    }

    pub fn position(&self, offset: &Self) -> Position {
        match self.cmp(offset) {
            Ordering::Less => Position::Earlier,
            Ordering::Equal => Position::Latest,
            Ordering::Greater => Position::Above,
        }
    }
}
