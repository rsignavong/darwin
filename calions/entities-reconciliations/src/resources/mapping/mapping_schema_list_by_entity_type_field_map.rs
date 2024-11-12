use super::{MappingFieldEntityType, MappingSchema, MappingSchemaList, MappingSchemaMethods};
use crate::decoders::{Mappings, RelationshipEntity};
use crate::resources::ResourcesError;
use crate::resources::{GraphEdgePoint, RelationshipId};
use ahash::AHashMap;
use std::convert::TryFrom;
use std::sync::Arc;

type MappingSchemaListByEntityTypeFieldHashMap =
    AHashMap<MappingFieldEntityType, MappingSchemaList>;

#[derive(Default)]
pub struct MappingSchemaListByEntityTypeFieldMap(MappingSchemaListByEntityTypeFieldHashMap);

impl TryFrom<&Mappings> for MappingSchemaListByEntityTypeFieldMap {
    type Error = ResourcesError;

    fn try_from(mappings: &Mappings) -> Result<Self, Self::Error> {
        fn add_mapping(
            map: &mut MappingSchemaListByEntityTypeFieldHashMap,
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
                    map.get_mut(&MappingFieldEntityType::new(field.clone(), type_.clone()))
                {
                    schemas.add(schema);
                } else {
                    map.insert(
                        MappingFieldEntityType::new(field.clone(), type_),
                        MappingSchemaList::new(schema),
                    );
                }
            }
        }

        let mut map = MappingSchemaListByEntityTypeFieldHashMap::new();

        for relationship in mappings.relationships.iter() {
            let rid = RelationshipId::new()?;

            add_mapping(
                &mut map,
                Arc::new(GraphEdgePoint::Head(rid.clone())),
                &relationship.entity,
            );
            add_mapping(
                &mut map,
                Arc::new(GraphEdgePoint::Tail(rid)),
                &relationship.belongs_to,
            );
        }

        Ok(MappingSchemaListByEntityTypeFieldMap(map))
    }
}

impl MappingSchemaListByEntityTypeFieldMap {
    pub fn get(&self, entity_type_field: &MappingFieldEntityType) -> Option<&MappingSchemaList> {
        self.0.get(entity_type_field)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
