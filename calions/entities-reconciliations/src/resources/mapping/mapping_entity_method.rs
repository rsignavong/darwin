use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MappingEntityMethod {
    Append,
    Fixed,
    Upsert,
}
