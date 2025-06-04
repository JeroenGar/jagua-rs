use std::hash::Hash;

use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::SPolygon;

/// Defines a set of edges from a hazard that is partially active in the [`QTNode`](crate::collision_detection::quadtree::QTNode).
#[derive(Clone, Debug)]
pub struct QTHazPartial {
    pub shape: SPolygon,
    pub edges: RelevantEdges,
}

impl QTHazPartial {
    pub fn all_edges(&self) -> bool {
        self.edges == RelevantEdges::All
    }

    pub fn register_edge(&mut self, index: usize) {
        match &mut self.edges {
            RelevantEdges::All => panic!("cannot add edge to a hazard that encompasses all edges"),
            RelevantEdges::Some(indices) => {
                indices.push(index);
            }
        }
    }

    pub fn n_edges(&self) -> usize {
        match &self.edges {
            RelevantEdges::All => self.shape.n_vertices(),
            RelevantEdges::Some(indices) => indices.len(),
        }
    }
}

impl<T: QTQueryable> CollidesWith<T> for QTHazPartial {
    fn collides_with(&self, entity: &T) -> bool {
        // If the entity does not collide with the bounding box of the hazard, it cannot collide with the hazard
        match entity.collides_with(&self.shape.bbox) {
            false => false,
            true => match &self.edges {
                RelevantEdges::All => self.shape.edge_iter().any(|e| entity.collides_with(&e)),
                RelevantEdges::Some(idxs) => idxs
                    .iter()
                    .map(|i| self.shape.edge(*i))
                    .any(|e| entity.collides_with(&e)),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum RelevantEdges {
    /// All edges of the hazard are relevant for the node
    All,
    /// Only some specific indices of the hazard's edges are relevant for the node
    Some(Vec<usize>),
}

impl From<usize> for RelevantEdges {
    fn from(index: usize) -> Self {
        RelevantEdges::Some(vec![index])
    }
}

impl From<Vec<usize>> for RelevantEdges {
    fn from(indices: Vec<usize>) -> Self {
        RelevantEdges::Some(indices)
    }
}
