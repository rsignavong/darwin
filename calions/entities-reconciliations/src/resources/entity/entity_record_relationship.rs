use super::EntityRecordPathPosition;
use crate::decoders::EntityRecordBody;
use crate::resources::MappingField;
use std::sync::Arc;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct EntityRecordRelationship(String);

impl EntityRecordRelationship {
    pub fn new(
        body: &EntityRecordBody,
        path: &[String],
        field: &Arc<MappingField>,
    ) -> Option<Self> {
        let position = EntityRecordPathPosition::new(&path, field);
        if position.is_not_found() {
            return None;
        }

        let value = body.get(format!("/{p}", p = path.join("/")).as_str())?;
        if !value.is_string() {
            return None;
        }

        Some(EntityRecordRelationship(value.as_str()?.into()))
    }
}
