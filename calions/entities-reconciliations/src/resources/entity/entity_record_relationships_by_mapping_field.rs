use super::EntityType;
use super::{EntitiyRecordRelationshipList, EntityRecordRelationship};
use super::{EntityRecordMetadata, EntityRecordMetadataList};
use crate::decoders::EntityRecordBody;
use crate::resources::{
    MappingField, MappingFieldEntityType, MappingSchemaListByEntityTypeFieldMap,
};
use ahash::AHashMap;
use std::sync::Arc;

type FieldRelationshipListMap = AHashMap<Arc<MappingField>, EntitiyRecordRelationshipList>;

pub struct EntityRecordRelationshipListByMappingFieldMap(FieldRelationshipListMap);

impl EntityRecordRelationshipListByMappingFieldMap {
    pub fn new(
        fields: &[Arc<MappingField>],
        keys: &[Vec<String>],
        body: &EntityRecordBody,
    ) -> Self {
        let mut relationships_by_field = FieldRelationshipListMap::new();
        for field in fields.iter() {
            for path in keys.iter() {
                let relationship = EntityRecordRelationship::new(body, path, field);

                if let Some(rel) = relationship {
                    let rel = Arc::new(rel);
                    if let Some(relationships) = relationships_by_field.get_mut(field) {
                        relationships.add(rel);
                    } else {
                        relationships_by_field
                            .insert(field.clone(), EntitiyRecordRelationshipList::new(rel));
                    }
                }
            }
        }

        EntityRecordRelationshipListByMappingFieldMap(relationships_by_field)
    }

    pub fn get_entity_records_metadata(
        self,
        mapping: &MappingSchemaListByEntityTypeFieldMap,
        type_: &Arc<EntityType>,
    ) -> EntityRecordMetadataList {
        let mut entity_records_metadata_list = EntityRecordMetadataList::new();
        for (field, relationships) in self.0.into_iter() {
            let entity_type_field = MappingFieldEntityType::new(type_.clone(), field);
            if let Some(schemas) = mapping.get_schemas_by_entity_type_field(&entity_type_field) {
                relationships.for_each_relationship(|relationship| {
                    schemas.for_each_schema(|schema| {
                        let metadata =
                            EntityRecordMetadata::new(field, relationship, schema.clone());
                        entity_records_metadata_list.add_metadata(metadata);
                    })
                });
            }
        }

        entity_records_metadata_list
    }
}
