use super::{MappingFieldEntityType, MappingSchema, MappingSchemaList, MappingSchemaMethods};
use crate::decoders::{Mappings, RelationshipEntity};
use crate::resources::ResourcesError;
use crate::resources::{GraphEdgePoint, RelationshipId};
use ahash::AHashMap;
use std::convert::TryFrom;
use std::sync::Arc;

type MappingHashMap = AHashMap<MappingFieldEntityType, MappingSchemaList>;

pub struct MappingSchemaListByEntityTypeFieldMap(MappingHashMap);

impl TryFrom<&Mappings> for MappingSchemaListByEntityTypeFieldMap {
    type Error = ResourcesError;

    fn try_from(mappings: &Mappings) -> Result<Self, Self::Error> {
        fn add_mapping(
            mapping: &mut MappingHashMap,
            edge_point: Arc<GraphEdgePoint>,
            relationship: &RelationshipEntity,
        ) {
            for field in relationship.matching_mappings.iter() {
                let type_ = relationship.type_.clone();
                let entity = relationship.method.clone();
                let relationship = relationship.relationship_method.clone();
                let schema = MappingSchema::new(
                    edge_point.clone(),
                    MappingSchemaMethods::new(entity, relationship),
                );
                if let Some(schemas) =
                    mapping.get_mut(&MappingFieldEntityType::new(type_.clone(), field.clone()))
                {
                    schemas.add(schema);
                } else {
                    mapping.insert(
                        MappingFieldEntityType::new(type_, field.clone()),
                        MappingSchemaList::new(schema),
                    );
                }
            }
        }

        let mut mapping = MappingHashMap::new();

        for relationship in mappings.relationships.iter() {
            let rid = RelationshipId::new()?;

            add_mapping(
                &mut mapping,
                Arc::new(GraphEdgePoint::Head(rid.clone())),
                &relationship.entity,
            );
            add_mapping(
                &mut mapping,
                Arc::new(GraphEdgePoint::Tail(rid)),
                &relationship.belongs_to,
            );
        }

        Ok(MappingSchemaListByEntityTypeFieldMap(mapping))
    }
}

impl MappingSchemaListByEntityTypeFieldMap {
    pub fn get_schemas_by_entity_type_field(
        &self,
        entity_type_field: &MappingFieldEntityType,
    ) -> Option<&MappingSchemaList> {
        self.0.get(entity_type_field)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
