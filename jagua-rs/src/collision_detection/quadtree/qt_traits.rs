use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::{Circle, Edge, Point};

use super::assertions;

/// Common trait for all geometric primitives that can be directly queried in the quadtree
/// for collisions with the edges of the registered hazards. These include: [Rect], [Edge] and [Circle].
pub trait QTQueryable: CollidesWith<Edge> + CollidesWith<Rect> {
    /// Checks
    fn collides_with_quadrants(&self, r: &Rect, qs: [&Rect; 4]) -> [bool; 4] {
        debug_assert!(r.quadrants().iter().zip(qs.iter()).all(|(q, r)| *q == **r));
        qs.map(|q| self.collides_with(q))
    }

    /// Returns the preferred order for searching the quadrants around `split`.
    /// The result must contain every index in `0..4` exactly once.
    fn quadrant_order(&self, _split: Point) -> [usize; 4] {
        [0, 1, 2, 3]
    }
}

impl QTQueryable for Circle {
    fn quadrant_order(&self, Point(mid_x, mid_y): Point) -> [usize; 4] {
        let right = self.center.x() >= mid_x;
        let top = self.center.y() >= mid_y;
        let row_mask = usize::from(!top) << 1;
        let containing_quadrant = row_mask | usize::from(right ^ top);
        let first_adjacent_mask = row_mask | 1;
        let order = [
            containing_quadrant,
            containing_quadrant ^ first_adjacent_mask,
            containing_quadrant ^ (first_adjacent_mask ^ 2),
            containing_quadrant ^ 2,
        ];
        debug_assert_eq!(
            order,
            match (right, top) {
                (true, true) => [0, 1, 3, 2],
                (false, true) => [1, 0, 2, 3],
                (false, false) => [2, 1, 3, 0],
                (true, false) => [3, 0, 2, 1],
            }
        );
        order
    }
}
impl QTQueryable for Rect {}

impl QTQueryable for Edge {
    fn collides_with_quadrants(&self, r: &Rect, qs: [&Rect; 4]) -> [bool; 4] {
        debug_assert!(r.quadrants().iter().zip(qs.iter()).all(|(q, r)| *q == **r));
        let classified = qs.map(|q| self.collides_with(q));
        debug_assert!(
            assertions::edge_quadrant_classification_is_conservative(self, qs, classified),
            "edge: {self:?}, node: {r:?}, quadrants: {qs:?}, classified: {classified:?}, f64 oracle: {:?}",
            assertions::edge_collisions_with_rects_f64(self, qs)
        );
        classified
    }
}
