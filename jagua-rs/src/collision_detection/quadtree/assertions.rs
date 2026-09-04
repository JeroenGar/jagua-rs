use crate::geometry::primitives::{Edge, Rect};

pub(super) fn edge_quadrant_classification_is_conservative(
    edge: &Edge,
    quadrants: [&Rect; 4],
    classified: [bool; 4],
) -> bool {
    edge_collisions_with_rects_f64(edge, quadrants)
        .into_iter()
        .zip(classified)
        .all(|(collides, classified)| !collides || classified)
}

pub(super) fn edge_collisions_with_rects_f64(edge: &Edge, rects: [&Rect; 4]) -> [bool; 4] {
    rects.map(|rect| edge_collides_with_rect_f64(edge, rect))
}

/// Recomputes the segment-rectangle intersection in `f64` so assertions do not inherit
/// conservative rounding from the production `f32` predicate.
#[allow(clippy::similar_names)]
fn edge_collides_with_rect_f64(edge: &Edge, rect: &Rect) -> bool {
    let (start_x, start_y) = (f64::from(edge.start.0), f64::from(edge.start.1));
    let (end_x, end_y) = (f64::from(edge.end.0), f64::from(edge.end.1));
    let (x_min, y_min) = (f64::from(rect.x_min), f64::from(rect.y_min));
    let (x_max, y_max) = (f64::from(rect.x_max), f64::from(rect.y_max));

    let x_no_overlap = start_x.min(end_x).max(x_min) > start_x.max(end_x).min(x_max);
    let y_no_overlap = start_y.min(end_y).max(y_min) > start_y.max(end_y).min(y_max);
    if x_no_overlap || y_no_overlap {
        return false;
    }

    let point_in_rect = |x, y| x >= x_min && x <= x_max && y >= y_min && y <= y_max;
    if point_in_rect(start_x, start_y) || point_in_rect(end_x, end_y) {
        return true;
    }

    let edge_dx = end_x - start_x;
    let edge_dy = end_y - start_y;
    let sides = [
        (x_max - start_x) * edge_dy - (y_max - start_y) * edge_dx,
        (x_min - start_x) * edge_dy - (y_max - start_y) * edge_dx,
        (x_min - start_x) * edge_dy - (y_min - start_y) * edge_dx,
        (x_max - start_x) * edge_dy - (y_min - start_y) * edge_dx,
    ];

    !sides.iter().all(|side| *side > 0.0) && !sides.iter().all(|side| *side < 0.0)
}
