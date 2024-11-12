use super::{ReconciliationCount, ReconciliationOrder};
use crate::decoders::ReconciliationCommandBody;
use crate::decoders::{EntityRecordBody, EntityRecordMetadata};
use crate::resources::AggregateMap;
use crate::resources::EntityRecordRelationshipListByMappingFieldMap;
use crate::resources::GraphMap;
use crate::resources::MappingId;
use crate::resources::Record;
use crate::resources::{EntityRecordId, EntityType};
use crate::resources::{MappingFieldListByEntityTypeMap, MappingSchemaListByEntityTypeFieldMap};
use crate::resources::{NodeAdjacencyListMap, NodeChanges, NodeEntityMaps};
use crate::resources::{ProcessorId, ResourcesError, StatusState};
use derive_new::new;
use log::{info, warn};
use serde_json::Value;
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(new)]
pub struct Reconciliation {
    #[new(default)]
    aggregate_map: Option<AggregateMap>,
    #[new(default)]
    count: ReconciliationCount,
    #[new(default)]
    graph_map: Option<GraphMap>,
    #[new(default)]
    mapping_fields_by_type_map: MappingFieldListByEntityTypeMap,
    #[new(default)]
    mapping_id: Option<Arc<MappingId>>,
    #[new(default)]
    mapping_schemas_by_type_field_map: MappingSchemaListByEntityTypeFieldMap,
    #[new(default)]
    node_adjacency_list_map: Option<NodeAdjacencyListMap>,
    #[new(default)]
    node_entity_maps: Option<NodeEntityMaps>,
    #[new(default)]
    order: Option<ReconciliationOrder>,
    processor_id: Arc<ProcessorId>,
}

impl Reconciliation {
    pub fn configure(&mut self, cmd_body: &ReconciliationCommandBody) -> Option<Arc<MappingId>> {
        if !self.processor_id.eq(&cmd_body.processor_id) {
            return None;
        }

        let some_mapping_id = Some(cmd_body.activated_mapping_id.clone());
        self.aggregate_map = Some(AggregateMap::new());
        self.graph_map = Some(GraphMap::new());
        self.mapping_fields_by_type_map =
            MappingFieldListByEntityTypeMap::try_from(&cmd_body.mappings).ok()?;
        self.mapping_id = some_mapping_id.clone();
        self.mapping_schemas_by_type_field_map =
            MappingSchemaListByEntityTypeFieldMap::try_from(&cmd_body.mappings).ok()?;
        self.node_adjacency_list_map = Some(NodeAdjacencyListMap::new());
        self.node_entity_maps = Some(NodeEntityMaps::new());
        self.order = Some(ReconciliationOrder::new(
            cmd_body.reconciliations.entities.clone(),
        ));

        info!(
            "Activated MappingSchemaListByEntityTypeFieldMap: {}",
            cmd_body.activated_mapping_id
        );
        some_mapping_id
    }

    fn deep_keys(value: &Value, current_path: Vec<String>, output: &mut Vec<Vec<String>>) {
        if current_path.len() > 0 {
            output.push(current_path.clone());
        }

        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let mut new_path = current_path.clone();
                    new_path.push(k.to_owned());
                    Self::deep_keys(v, new_path, output);
                }
            }
            Value::Array(array) => {
                for (i, v) in array.iter().enumerate() {
                    let mut new_path = current_path.clone();
                    new_path.push(i.to_string().to_owned());
                    Self::deep_keys(v, new_path, output);
                }
            }
            _ => (),
        }
    }

    pub fn delete(
        &mut self,
        type_: &Arc<EntityType>,
        body: &EntityRecordBody,
        metadata: &EntityRecordMetadata,
    ) -> Result<Option<Vec<Record>>, ResourcesError> {
        let mut aggregate_map = self
            .aggregate_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationDeleteMissingAggregates)?;
        let graph_map = self
            .graph_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationDeleteMissingGraph)?;
        let mapping_id = self
            .mapping_id
            .as_ref()
            .ok_or_else(|| ResourcesError::ReconciliationDeleteMissingActivatedMappingId)?;
        let mut node_adjacency_list_map = self
            .node_adjacency_list_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingAdjacencies)?;
        let node_entity_maps = self
            .node_entity_maps
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingNodes)?;
        let order = self
            .order
            .as_ref()
            .ok_or_else(|| ResourcesError::ReconciliationDeleteMissingOrder)?;

        if let Some(activated_mapping_id) = &metadata.activated_mapping_id {
            if !mapping_id.eq(&activated_mapping_id) {
                warn!(
                    "Wrong ActivatedMappingId {:?}, expecting {:?}",
                    metadata.activated_mapping_id, mapping_id
                );
                return Ok(None);
            }
        }

        let record_id = if let Some(id) = body.get("/id") {
            EntityRecordId::try_from(id)?
        } else {
            warn!("Missing Id in record");
            return Ok(None);
        };

        let node_entity = if let Some(node_entity) = node_entity_maps.lookup(record_id, type_) {
            node_entity
        } else {
            return Ok(None);
        };

        let graph_changes = graph_map.delete(&node_entity);
        let node_changes = NodeChanges::from(graph_changes);
        let deleted_list = node_changes.unique_deleted_list();
        let deleted_aggregates = deleted_list.aggregate(
            &mut aggregate_map,
            &mut node_adjacency_list_map,
            &node_changes,
            &node_entity_maps,
            order,
        )?;

        self.count.set(aggregate_map.len() as u64);

        let records = deleted_aggregates.map_records(mapping_id, &self.processor_id, &self.count);

        node_entity_maps.delete(node_entity);

        Ok(Some(records))
    }

    pub fn is_ready(&self) -> bool {
        let ready = self.aggregate_map.is_some()
            && self.graph_map.is_some()
            && self.mapping_id.is_some()
            && !self.mapping_schemas_by_type_field_map.is_empty()
            && !self.mapping_fields_by_type_map.is_empty();
        info!(
            "Reconciliations is {}",
            if ready { "ready" } else { "not ready..." }
        );
        ready
    }

    pub fn match_record(
        &mut self,
        type_: &Arc<EntityType>,
        body: &EntityRecordBody,
        metadata: &EntityRecordMetadata,
    ) -> Result<Option<Vec<Record>>, ResourcesError> {
        let mut aggregate_map = self
            .aggregate_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingAggregateMap)?;
        let fields = self
            .mapping_fields_by_type_map
            .get_fields(type_)
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingFields)?;
        let graph_map = self
            .graph_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingGraph)?;
        let mapping_id = self
            .mapping_id
            .as_ref()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingActivatedMappingId)?;
        let mut node_adjacency_list_map = self
            .node_adjacency_list_map
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingAdjacencies)?;
        let node_entity_maps = self
            .node_entity_maps
            .as_mut()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingNodes)?;
        let order = self
            .order
            .as_ref()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingOrder)?;

        if let Some(activated_mapping_id) = &metadata.activated_mapping_id {
            if !mapping_id.eq(&activated_mapping_id) {
                warn!(
                    "Wrong ActivatedMappingId {:?}, expecting {:?}",
                    metadata.activated_mapping_id, mapping_id
                );
                return Ok(None);
            }
        }

        let record_id = if let Some(id) = body.get("/id") {
            EntityRecordId::try_from(id)?
        } else {
            warn!("Missing Id in record");
            return Ok(None);
        };

        let node_entity = node_entity_maps.lookup_or_create(record_id, type_)?;

        let mut record_keys = vec![vec![]];
        let current_path = vec![];
        Self::deep_keys(body.value(), current_path, &mut record_keys);
        let relationships_by_field =
            EntityRecordRelationshipListByMappingFieldMap::new(body, fields, &record_keys);

        let graph_changes = graph_map.upsert(
            &node_entity,
            relationships_by_field,
            &self.mapping_schemas_by_type_field_map,
        );
        let node_changes = NodeChanges::from(graph_changes);
        let deleted_list = node_changes.unique_deleted_list();
        let deleted_aggregates = deleted_list.aggregate(
            &mut aggregate_map,
            &mut node_adjacency_list_map,
            &node_changes,
            &node_entity_maps,
            order,
        )?;

        let created_list = node_changes.unique_created_list();

        let mut created_aggregates = created_list.aggregate(
            &mut aggregate_map,
            &mut node_adjacency_list_map,
            &node_changes,
            &node_entity_maps,
            order,
        )?;
        created_aggregates.extend(deleted_aggregates);

        self.count.set(aggregate_map.len() as u64);

        let records = created_aggregates.map_records(mapping_id, &self.processor_id, &self.count);

        Ok(Some(records))
    }

    pub fn set_temp_state(&mut self) -> StatusState {
        info!("Temporary Fake Reload Reconciliation data");

        let aggregate_map = AggregateMap::new();
        let graph_map = GraphMap::new();
        // Reading path and insert
        //     self.profiles
        //         .insert(tesla_composed_key.to_owned(), profile_id);
        // file must contains count
        // if last one equql self.count => ok
        // compare length of profiles with self.process_count
        // if error => None
        self.aggregate_map = Some(aggregate_map);
        self.graph_map = Some(graph_map);

        info!("Reconciliation state reloaded");
        StatusState::Ready
    }
}
