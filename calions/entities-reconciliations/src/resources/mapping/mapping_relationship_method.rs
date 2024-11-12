use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MappingRelationshipMethod {
    Append,
    Fixed,
    Upsert,
}
