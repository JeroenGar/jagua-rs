use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::quadtree::edge_interval_iter::EdgeIntervalIterator;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Clone, Debug)]
pub struct QTPartialHazard {
    shape: Weak<SimplePolygon>,
    presence: GeoPosition,
    intervals: Vec<(usize, usize)>,
}

impl From<&Hazard> for QTPartialHazard {
    fn from(hazard: &Hazard) -> Self {
        Self {
            shape: Arc::downgrade(hazard.shape()),
            presence: hazard.entity().presence(),
            intervals: vec![(0, 0)],
        }
    }
}

impl QTPartialHazard {
    pub fn new(shape: Weak<SimplePolygon>, presence: GeoPosition, intervals: Vec<(usize, usize)>) -> Self {
        Self {
            shape,
            presence,
            intervals,
        }
    }

    pub fn shape(&self) -> &Weak<SimplePolygon> {
        &self.shape
    }

    pub fn position(&self) -> GeoPosition {
        self.presence
    }

    pub fn intervals(&self) -> &[(usize, usize)] {
        &self.intervals
    }
}

impl CollidesWith<Edge> for QTPartialHazard {
    fn collides_with(&self, edge: &Edge) -> bool {
        let shape = self.shape.upgrade().expect("polygon reference is not alive");
        let n_points = shape.number_of_points();

        //check if any edge from any interval collides with the given edge
        self.intervals.iter().any(|interval| {
            EdgeIntervalIterator::new(*interval, n_points)
                .map(|pair| shape.get_edge(pair.0, pair.1))
                .any(|shape_edge| edge.collides_with(&shape_edge))
        })
    }
}

impl CollidesWith<Circle> for QTPartialHazard {
    fn collides_with(&self, circle: &Circle) -> bool {
        let shape = self.shape.upgrade().expect("polygon reference is not alive");
        let n_points = shape.number_of_points();

        self.intervals.iter().any(|interval| {
            EdgeIntervalIterator::new(*interval, n_points)
                .map(|pair| shape.get_edge(pair.0, pair.1))
                .any(|shape_edge| circle.collides_with(&shape_edge))
        })
    }
}

impl PartialEq for QTPartialHazard {
    fn eq(&self, other: &Self) -> bool {
        self.presence == other.presence &&
            self.intervals == other.intervals
    }
}

impl Eq for QTPartialHazard {}

impl Hash for QTPartialHazard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.presence.hash(state);
        self.intervals.hash(state);
    }
}
