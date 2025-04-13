use crate::fsize;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::SimplePolygon;
use ordered_float::OrderedFloat;

/// Returns the indices of the points in the [`SimplePolygon`] that form the convex hull
pub fn convex_hull_indices(shape: &SimplePolygon) -> Vec<usize> {
    let c_hull = convex_hull_from_points(shape.points.clone());
    let mut indices = vec![];
    for p in c_hull.iter() {
        indices.push(shape.points.iter().position(|x| x == p).unwrap());
    }
    indices
}

/// Reconstitutes the convex hull of a [`SimplePolygon`] using its surrogate
pub fn convex_hull_from_surrogate(s: &SimplePolygon) -> Result<Vec<Point>, &'static str> {
    if let Some(surr) = s.surrogate.as_ref() {
        Ok(surr
            .convex_hull_indices
            .iter()
            .map(|&i| s.points[i])
            .collect())
    } else {
        Err("No surrogate available")
    }
}

/// Filters a set of points to only include those that are part of the convex hull
pub fn convex_hull_from_points(mut points: Vec<Point>) -> Vec<Point> {
    //https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain

    //sort the points by x coordinate
    points.sort_by_key(|p| OrderedFloat(p.0));

    let mut lower_hull = points
        .iter()
        .fold(vec![], |hull, p| grow_convex_hull(hull, *p));
    let mut upper_hull = points
        .iter()
        .rev()
        .fold(vec![], |hull, p| grow_convex_hull(hull, *p));

    //First and last element of both hull parts are the same point
    upper_hull.pop();
    lower_hull.pop();

    lower_hull.append(&mut upper_hull);
    lower_hull
}

fn grow_convex_hull(mut h: Vec<Point>, next: Point) -> Vec<Point> {
    //pop all points from the hull which will be made irrelevant due to the new point
    while h.len() >= 2 && cross(h[h.len() - 2], h[h.len() - 1], next) <= 0.0 {
        h.pop();
    }
    h.push(next);
    h
}

fn cross(a: Point, b: Point, c: Point) -> fsize {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}
