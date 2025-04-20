use std::hash::Hash;
use std::sync::{Arc, Weak};

use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::SPolygon;

/// Defines a set of edges from a hazard that is partially active in the [`QTNode`](crate::collision_detection::quadtree::QTNode).
#[derive(Clone, Debug)]
pub struct QTHazPartial {
    pub shape: Weak<SPolygon>,
    pub edges: RelevantEdges,
}

impl QTHazPartial {
    pub fn shape_arc(&self) -> Arc<SPolygon> {
        self.shape
            .upgrade()
            .expect("polygon reference is not alive")
    }

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
}

//check bbox if number of edges is this or greater
const BBOX_CHECK_THRESHOLD: usize = 10;

const BBOX_CHECK_THRESHOLD_MINUS_1: usize = BBOX_CHECK_THRESHOLD - 1;

impl<T: QTQueryable> CollidesWith<T> for QTHazPartial {
    fn collides_with(&self, entity: &T) -> bool {
        let shape = self.shape_arc();
        match &self.edges {
            RelevantEdges::All => match entity.collides_with(&shape.bbox) {
                false => false,
                true => shape.edge_iter().any(|e| entity.collides_with(&e)),
            },
            RelevantEdges::Some(indices) => match indices.len() {
                0 => unreachable!("edge indices should not be empty"),
                1..=BBOX_CHECK_THRESHOLD_MINUS_1 => indices
                    .iter()
                    .any(|&i| entity.collides_with(&shape.edge(i))),
                BBOX_CHECK_THRESHOLD.. => {
                    if !entity.collides_with(&shape.bbox) {
                        return false;
                    }
                    indices
                        .iter()
                        .any(|&i| entity.collides_with(&shape.edge(i)))
                }
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
