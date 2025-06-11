use crate::collision_detection::quadtree::qt_traits::QTQueryable;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::{Edge, Rect, SPolygon};
use std::sync::Arc;

/// Defines a set of edges from a hazard that is partially active in the [`QTNode`](crate::collision_detection::quadtree::QTNode).
#[derive(Clone, Debug)]
pub struct QTHazPartial {
    /// A reference to the full shape of the hazard
    pub shape: Arc<SPolygon>,
    /// The edges that are active in the quadtree-node.
    pub edges: Vec<Edge>,
    /// A bounding box that guarantees all edges are contained within it. (used for fail fast)
    pub ff_bbox: Rect,
}

impl QTHazPartial {
    pub fn new(shape: Arc<SPolygon>, edges: Vec<Edge>) -> Self {
        debug_assert!(!edges.is_empty());
        let ff_bbox = {
            //calculate a bounding box around the edges
            if edges.len() == shape.n_vertices() {
                // If the edges cover the entire shape, use the shape's bounding box
                shape.bbox
            } else {
                // Otherwise, calculate the bounding box from the edges
                let (mut x_min, mut y_min, mut x_max, mut y_max) = (
                    f32::INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::NEG_INFINITY,
                );
                for edge in &edges {
                    x_min = x_min.min(edge.start.x()).min(edge.end.x());
                    y_min = y_min.min(edge.start.y()).min(edge.end.y());
                    x_max = x_max.max(edge.start.x()).max(edge.end.x());
                    y_max = y_max.max(edge.start.y()).max(edge.end.y());
                }
                if x_min < x_max && y_min < y_max {
                    Rect {
                        x_min,
                        y_min,
                        x_max,
                        y_max,
                    }
                } else {
                    // If the edges are all aligned to an axis, use the shape's bounding box
                    shape.bbox
                }
            }
        };

        Self {
            shape,
            edges,
            ff_bbox,
        }
    }

    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }
}

impl<T: QTQueryable> CollidesWith<T> for QTHazPartial {
    fn collides_with(&self, entity: &T) -> bool {
        // If the entity does not collide with the bounding box of the hazard, it cannot collide with the hazard
        entity.collides_with(&self.ff_bbox) && self.edges.iter().any(|e| entity.collides_with(e))
    }
}
