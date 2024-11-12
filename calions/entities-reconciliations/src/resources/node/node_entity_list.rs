use super::NodeEntity;
use crate::resources::{AggregateSet, ReconciliationOrder, RecordReconciliations};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeEntityList(Vec<NodeEntity>);

impl NodeEntityList {
    pub fn extract(
        self,
        aggregates: &mut AggregateSet,
        reconciliations: &mut RecordReconciliations,
    ) {
        for ne in self.0.into_iter() {
            aggregates.add_node_id(ne.node_id());
            reconciliations.add_entity(ne.entity())
        }
    }

    pub fn is_same_size(&self, order: &ReconciliationOrder) -> bool {
        self.0.len() == order.len()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new(nes: &[NodeEntity], order: &ReconciliationOrder) -> Self {
        let size = order.len();
        let mut node_entities = Vec::with_capacity(size);
        order.for_each_type(|type_| {
            for ne in nes.iter() {
                if ne.is_same_type(type_) {
                    node_entities.push(ne.clone());
                    return;
                }
            }
        });

        NodeEntityList(node_entities)
    }
}
