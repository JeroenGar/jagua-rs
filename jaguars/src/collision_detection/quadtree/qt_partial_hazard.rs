use std::borrow::Borrow;
use std::hash::{Hash};
use std::sync::{Arc, Weak};

use crate::collision_detection::hazard::Hazard;
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::simple_polygon::SimplePolygon;


/// QTPartialHazards define a set of edges from a hazard that cross the QTNode.
#[derive(Clone, Debug)]
pub struct QTPartialHazard {
    shape: Weak<SimplePolygon>,
    edge_indices: EdgeIndices,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum EdgeIndices {
    All,
    Some(Vec<usize>)
}

impl<T> From<T> for QTPartialHazard where T: Borrow<Hazard> {
    fn from(hazard: T) -> Self {
        Self {
            shape: Arc::downgrade(&hazard.borrow().shape),
            edge_indices: EdgeIndices::All,
        }
    }
}

impl QTPartialHazard {

    pub fn new(shape: Arc<SimplePolygon>, edge_indices: EdgeIndices) -> Self {
        Self {
            shape: Arc::downgrade(&shape),
            edge_indices,
        }
    }

    pub fn shape_weak(&self) -> &Weak<SimplePolygon> {
        &self.shape
    }

    pub fn shape(&self) -> Arc<SimplePolygon> {
        self.shape.upgrade().expect("polygon reference is not alive")
    }

    pub fn edge_indices(&self) -> &EdgeIndices{
        &self.edge_indices
    }

    pub fn encompasses_all_edges(&self) -> bool {
        self.edge_indices == EdgeIndices::All
    }

    pub fn add_edge_index(&mut self, index: usize) {
        match &mut self.edge_indices {
            EdgeIndices::All => panic!("cannot add edge to a hazard that encompasses all edges"),
            EdgeIndices::Some(indices) => {
                indices.push(index);
            }
        }
    }

}
impl CollidesWith<Edge> for QTPartialHazard {
    fn collides_with(&self, edge: &Edge) -> bool {
        let shape = self.shape.upgrade().expect("polygon reference is not alive");
        match self.edge_indices() {
            EdgeIndices::All => {
                match shape.bbox().collides_with(edge) {
                    false => false,
                    true => shape.edge_iter().any(|e| {
                        edge.collides_with(&e)
                    })
                }
            },
            EdgeIndices::Some(indices) => {
                indices.iter().any(|&i| {
                    edge.collides_with(&shape.get_edge(i))
                })
            }
        }
    }
}

impl CollidesWith<Circle> for QTPartialHazard {
    fn collides_with(&self, circle: &Circle) -> bool {
        let shape = self.shape.upgrade().expect("polygon reference is not alive");
        match self.edge_indices() {
            EdgeIndices::All => {
                match circle.collides_with(&shape.bbox()) {
                    false => false,
                    true => shape.edge_iter().any(|e| {
                        circle.collides_with(&e)
                    })
                }
            },
            EdgeIndices::Some(indices) => {
                indices.iter().any(|&i| {
                    circle.collides_with(&shape.get_edge(i))
                })
            }
        }
    }
}
