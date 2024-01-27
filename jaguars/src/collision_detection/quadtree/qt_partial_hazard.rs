use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

use crate::collision_detection::hazards::hazard::Hazard;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Clone, Debug)]
pub struct QTPartialHazard {
    shape: Weak<SimplePolygon>,
    presence: GeoPosition,
    edge_indices: EdgeIndices,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum EdgeIndices {
    All,
    Some(Vec<usize>)
}

impl From<&Hazard> for QTPartialHazard {
    fn from(hazard: &Hazard) -> Self {
        Self {
            shape: Arc::downgrade(hazard.shape()),
            presence: hazard.entity().presence(),
            edge_indices: EdgeIndices::All,
        }
    }
}

impl QTPartialHazard {

    pub fn new(shape: Arc<SimplePolygon>, presence: GeoPosition, edge_indices: EdgeIndices) -> Self {
        Self {
            shape: Arc::downgrade(&shape),
            presence,
            edge_indices,
        }
    }

    pub fn shape_weak(&self) -> &Weak<SimplePolygon> {
        &self.shape
    }

    pub fn shape(&self) -> Arc<SimplePolygon> {
        self.shape.upgrade().expect("polygon reference is not alive")
    }

    pub fn presence(&self) -> GeoPosition {
        self.presence
    }

    pub fn edge_indices(&self) -> &EdgeIndices{
        &self.edge_indices
    }

    pub fn encompasses_all_edges(&self) -> bool {
        match self.edge_indices {
            EdgeIndices::All => true,
            EdgeIndices::Some(_) => false
        }
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
                shape.edge_iter().any(|e| {
                    edge.collides_with(&e)
                })
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
                shape.edge_iter().any(|e| {
                    circle.collides_with(&e)
                })
            },
            EdgeIndices::Some(indices) => {
                indices.iter().any(|&i| {
                    circle.collides_with(&shape.get_edge(i))
                })
            }
        }
    }
}

impl PartialEq for QTPartialHazard {
    fn eq(&self, other: &Self) -> bool {
        self.presence == other.presence &&
            self.edge_indices == other.edge_indices
    }
}

impl Eq for QTPartialHazard {}

impl Hash for QTPartialHazard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.presence.hash(state);
        self.edge_indices.hash(state);
    }
}
