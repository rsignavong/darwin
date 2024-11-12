use super::EntityType;
use super::{EntitiyRecordRelationshipList, EntityRecordRelationship};
use super::{EntityRecordMetadata, EntityRecordMetadataList};
use crate::decoders::EntityRecordBody;
use crate::resources::MappingSchemaListByEntityTypeFieldMap;
use crate::resources::{MappingField, MappingFieldEntityType, MappingFieldList};
use ahash::AHashMap;
use std::sync::Arc;

type EntityRecordRelationshipListByMappingFieldHashMap =
    AHashMap<Arc<MappingField>, EntitiyRecordRelationshipList>;

pub struct EntityRecordRelationshipListByMappingFieldMap(
    EntityRecordRelationshipListByMappingFieldHashMap,
);

impl EntityRecordRelationshipListByMappingFieldMap {
    pub fn new(body: &EntityRecordBody, fields: &MappingFieldList, keys: &[Vec<String>]) -> Self {
        let mut relationships_by_field = EntityRecordRelationshipListByMappingFieldHashMap::new();
        fields.iter_each(|field| {
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
        });

        EntityRecordRelationshipListByMappingFieldMap(relationships_by_field)
    }

    pub fn get_entity_record_metadata_list(
        self,
        schemas_by_type_field: &MappingSchemaListByEntityTypeFieldMap,
        type_: &Arc<EntityType>,
    ) -> EntityRecordMetadataList {
        let mut metadata_list = EntityRecordMetadataList::new();
        for (field, relationships) in self.0.into_iter() {
            let field_type = MappingFieldEntityType::new(field.clone(), type_.clone());
            if let Some(schemas) = schemas_by_type_field.get(&field_type) {
                relationships.for_each(|relationship| {
                    schemas.for_each(|schema| {
                        let metadata = EntityRecordMetadata::new(relationship.clone(), schema);
                        metadata_list.add(metadata);
                    })
                });
            }
        }

        metadata_list
    }
}
