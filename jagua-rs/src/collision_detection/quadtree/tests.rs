use super::assertions;
use super::qt_traits::QTQueryable;
use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::{Edge, Point, Rect};

#[test]
fn edge_quadrants_allow_f32_false_positive_at_bisector() {
    let rect = Rect::try_new(0.0, 6.1875, 100.0, 106.1875).unwrap();
    let quadrants = rect.quadrants();
    let edge = Edge {
        start: Point(-29.439_144, -32.354_836),
        end: Point(51.902_332, 212.291_02),
    };
    let bisector_y = rect.centroid().1;
    let crossing_fraction =
        -f64::from(edge.start.0) / (f64::from(edge.end.0) - f64::from(edge.start.0));
    let crossing_y = f64::from(edge.start.1)
        + crossing_fraction * (f64::from(edge.end.1) - f64::from(edge.start.1));
    let bisector_ulp = f32::from_bits(bisector_y.to_bits() + 1) - bisector_y;

    assert!(crossing_y > f64::from(bisector_y));
    assert!(crossing_y - f64::from(bisector_y) < f64::from(bisector_ulp));

    assert_eq!(
        quadrants.map(|quadrant| edge.collides_with(&quadrant)),
        [false, true, true, false]
    );
    assert_eq!(
        assertions::edge_collisions_with_rects_f64(&edge, quadrants.each_ref()),
        [false, true, false, false]
    );
    assert_eq!(
        edge.collides_with_quadrants(&rect, quadrants.each_ref()),
        [false, true, false, false]
    );
}
