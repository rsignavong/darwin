use super::ProfileId;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Profile {
    Old(ProfileId),
    New,
}

impl PartialOrd for Profile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Old(self_ulid), Self::Old(other_ulid)) => {
                if self_ulid < other_ulid {
                    return Some(Ordering::Less);
                }
                if self_ulid > other_ulid {
                    return Some(Ordering::Greater);
                }
                Some(Ordering::Equal)
            }
            (Self::New, Self::Old(_)) => Some(Ordering::Greater),
            (Self::Old(_), Self::New) => Some(Ordering::Less),
            (Self::New, Self::New) => Some(Ordering::Equal),
        }
    }
}

impl Ord for Profile {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Old(self_ulid), Self::Old(other_ulid)) => {
                if self_ulid < other_ulid {
                    return Ordering::Less;
                }
                if self_ulid > other_ulid {
                    return Ordering::Greater;
                }
                Ordering::Equal
            }
            (Self::New, Self::Old(_)) => Ordering::Greater,
            (Self::Old(_), Self::New) => Ordering::Less,
            (Self::New, Self::New) => Ordering::Equal,
        }
    }
}

