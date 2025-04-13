use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::{Arc, Weak};

use crate::collision_detection::hazards::Hazard;
use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::SimplePolygon;

/// Defines a set of edges from a hazard that is partially active in the [QTNode](crate::collision_detection::quadtree::qt_node::QTNode).
#[derive(Clone, Debug)]
pub struct PartialQTHaz {
    pub shape: Weak<SimplePolygon>,
    pub edges: RelevantEdges,
}

impl<T> From<T> for PartialQTHaz
where
    T: Borrow<Hazard>,
{
    fn from(hazard: T) -> Self {
        Self {
            shape: Arc::downgrade(&hazard.borrow().shape),
            edges: RelevantEdges::All,
        }
    }
}

impl PartialQTHaz {
    pub fn new(shape: Arc<SimplePolygon>, edge_indices: RelevantEdges) -> Self {
        Self {
            shape: Arc::downgrade(&shape),
            edges: edge_indices,
        }
    }

    pub fn shape_arc(&self) -> Arc<SimplePolygon> {
        self.shape
            .upgrade()
            .expect("polygon reference is not alive")
    }

    pub fn encompasses_all_edges(&self) -> bool {
        self.edges == RelevantEdges::All
    }

    pub fn add_edge_index(&mut self, index: usize) {
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

impl<T> CollidesWith<T> for PartialQTHaz
where
    T: QTQueryable,
{
    fn collides_with(&self, entity: &T) -> bool {
        let shape = self.shape_arc();
        match &self.edges {
            RelevantEdges::All => match entity.collides_with(&shape.bbox()) {
                false => false,
                true => shape.edge_iter().any(|e| entity.collides_with(&e)),
            },
            RelevantEdges::Some(indices) => match indices.len() {
                0 => unreachable!("edge indices should not be empty"),
                1..=BBOX_CHECK_THRESHOLD_MINUS_1 => indices
                    .iter()
                    .any(|&i| entity.collides_with(&shape.get_edge(i))),
                BBOX_CHECK_THRESHOLD.. => {
                    if !entity.collides_with(&shape.bbox()) {
                        return false;
                    }
                    indices
                        .iter()
                        .any(|&i| entity.collides_with(&shape.get_edge(i)))
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
