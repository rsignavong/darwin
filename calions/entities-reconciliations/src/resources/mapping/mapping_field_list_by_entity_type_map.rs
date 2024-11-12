use super::{MappingField, MappingFieldList};
use crate::decoders::Mappings;
use crate::resources::{EntityType, ResourcesError};
use ahash::AHashMap;
use std::convert::TryFrom;
use std::sync::Arc;

type MappingFieldListByEntityTypeHashMap = AHashMap<Arc<EntityType>, MappingFieldList>;

#[derive(Default)]
pub struct MappingFieldListByEntityTypeMap(MappingFieldListByEntityTypeHashMap);

impl TryFrom<&Mappings> for MappingFieldListByEntityTypeMap {
    type Error = ResourcesError;

    fn try_from(mappings: &Mappings) -> Result<Self, Self::Error> {
        fn insert_or_concat(
            map: &mut MappingFieldListByEntityTypeHashMap,
            type_: Arc<EntityType>,
            fields: Vec<Arc<MappingField>>,
        ) {
            if let Some(existing_fields) = map.get_mut(&type_) {
                for field in fields.iter() {
                    if !existing_fields.contains(field) {
                        existing_fields.add(field.clone());
                    }
                }
            } else {
                map.insert(type_, MappingFieldList::new(fields));
            }
        }

        let mut map = MappingFieldListByEntityTypeHashMap::new();
        for relationship in mappings.relationships.iter() {
            let type_ = relationship.entity.type_.clone();
            let fields = relationship.entity.matching_mappings.clone();
            insert_or_concat(&mut map, type_, fields);

            let type_ = relationship.belongs_to.type_.clone();
            let fields = relationship.belongs_to.matching_mappings.clone();
            insert_or_concat(&mut map, type_, fields);
        }

        Ok(MappingFieldListByEntityTypeMap(map))
    }
}

impl MappingFieldListByEntityTypeMap {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_fields(&self, type_: &Arc<EntityType>) -> Option<&MappingFieldList> {
        self.0.get(type_)
    }
}
