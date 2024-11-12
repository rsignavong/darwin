use crate::resources::RelationshipId;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphEdgePoint {
    Tail(RelationshipId),
    Head(RelationshipId),
}

impl GraphEdgePoint {
    pub fn opposite(&self) -> Self {
        match self {
            GraphEdgePoint::Tail(id) => GraphEdgePoint::Head(id.clone()),
            GraphEdgePoint::Head(id) => GraphEdgePoint::Tail(id.clone()),
        }
    }
}
