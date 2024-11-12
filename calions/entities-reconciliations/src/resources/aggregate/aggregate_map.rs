use super::{AggregateCount, AggregateCountMap, AggregateId, AggregateRc, AggregateSet};
use super::{AggregateEntry, AggregateEvent, AggregateEventList, AggregateRecord};
use crate::resources::ReconciliationOrder;
use crate::resources::ResourcesError;
// use crate::resources::{Entity, EntityType};
use crate::resources::{NodeChanges, NodeConnectedSet, NodeEntityList, NodeEntityMaps, NodeId};
use ahash::{AHashMap, AHashSet};
use derive_new::new;
use itertools::Itertools;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct AggregateMap(#[new(default)] AHashMap<AggregateSet, AggregateRc>);

impl AggregateMap {
    fn add(
        &mut self,
        aggregates: AggregateSet,
        count: AggregateCount,
    ) -> Result<Option<Arc<AggregateId>>, ResourcesError> {
        let id = if let Some(rc) = self.0.get_mut(&aggregates) {
            rc.set_count(count);
            None
        } else {
            let rc = AggregateRc::new(count)?;
            let id = rc.id().clone();
            self.0.insert(aggregates, rc);
            Some(id)
        };

        Ok(id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn remove(
        &mut self,
        aggregates: AggregateSet,
        count: AggregateCount,
    ) -> Option<Arc<AggregateId>> {
        if let Some(rc) = self.0.get_mut(&aggregates) {
            if rc.sub(count) {
                let id = rc.id().clone();
                self.0.remove(&aggregates);
                return Some(id);
            }

            return None;
        }
        None
    }

    pub fn save(
        &mut self,
        connected_set: NodeConnectedSet,
        nodes: &NodeEntityMaps,
        order: &ReconciliationOrder,
        delete_changes: Option<&NodeChanges>,
    ) -> Result<AggregateEventList, ResourcesError> {
        let nes_list: Vec<NodeEntityList> = connected_set
            .map_entity_and_group_by_type(&nodes)?
            .into_iter()
            .multi_cartesian_product()
            .filter_map(|node_entities| -> Option<NodeEntityList> {
                // in case of delete_changes,
                // we filter out those that don't contain the delete_set
                if let Some(changes) = delete_changes {
                    let node_entities_set: AHashSet<Arc<NodeId>> = node_entities
                        .iter()
                        .map(|ne| ne.node_id().clone())
                        .collect();
                    let mut contains_delete_tuple_set = false;
                    changes.for_each_delete(|change| {
                        let delete_tuple_set = change.tuple_set();
                        if !node_entities_set.is_disjoint(&delete_tuple_set) {
                            contains_delete_tuple_set = true;
                        }
                    });

                    if !contains_delete_tuple_set {
                        return None;
                    }
                }

                let nes = NodeEntityList::new(&node_entities, order);
                if !nes.is_same_size(order) {
                    return None;
                }

                Some(nes)
            })
            .collect();

        let mut aggregate_count_map = AggregateCountMap::new();
        for nes in nes_list.into_iter() {
            let arr = AggregateRecord::from(nes);
            let aggregates = arr.aggregates();

            aggregate_count_map.add(aggregates.clone(), arr.reconciliations().clone());
        }

        let mut events = AggregateEventList::new();
        aggregate_count_map.for_each(
            |aggregates, reconciliations, count| -> Result<(), ResourcesError> {
                if let Some(_) = delete_changes {
                    if let Some(id) = self.remove(aggregates.clone(), count) {
                        events.add_event(AggregateEvent::Deleted(AggregateEntry::new(
                            id.clone(),
                            reconciliations,
                        )));
                    }
                } else {
                    if let Some(id) = self.add(aggregates.clone(), count)? {
                        events.add_event(AggregateEvent::Created(AggregateEntry::new(
                            id,
                            reconciliations,
                        )));
                    }
                }

                Ok(())
            },
        )?;

        Ok(events)
    }
}
