use crate::resources::MappingField;
use std::sync::Arc;

pub enum EntityRecordPathPosition {
    BeforeLast,
    Last,
    NotFound,
}

impl EntityRecordPathPosition {
    pub fn is_not_found(&self) -> bool {
        match self {
            Self::NotFound => true,
            _ => false,
        }
    }

    pub fn new(path: &[String], predicate: &Arc<MappingField>) -> Self {
        match path.iter().position(|elt| *elt == predicate.to_string()) {
            Some(position) => {
                let len = path.len();
                match position {
                    p if p == len - 1 => EntityRecordPathPosition::Last,
                    p if p == len - 2 => {
                        if let Some(last) = path.last() {
                            if let Ok(_) = last.parse::<u32>() {
                                return EntityRecordPathPosition::BeforeLast;
                            }
                        }

                        EntityRecordPathPosition::NotFound
                    }
                    _ => EntityRecordPathPosition::NotFound,
                }
            }
            None => EntityRecordPathPosition::NotFound,
        }
    }
}
