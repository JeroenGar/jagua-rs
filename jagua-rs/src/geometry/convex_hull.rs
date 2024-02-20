use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

///<https://en.wikipedia.org/wiki/Convex_hull>

/// Returns the indices of the points in the SimplePolygon that form the convex hull
pub fn convex_hull_indices(shape: &SimplePolygon) -> Vec<usize> {
    let c_hull = convex_hull_from_points(shape.points.clone());
    let mut indices = vec![];
    for p in c_hull.iter() {
        indices.push(shape.points.iter().position(|x| x == p).unwrap());
    }
    indices
}

/// Returns the points that form the convex hull of the input points
/// Uses the Monotone chain algorithm : <https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain>
pub fn convex_hull_from_points(mut points: Vec<Point>) -> Vec<Point> {

    //sort the points by x coordinate
    points.sort_by(|a, b| {
        let (a_x, b_x) = (a.0, b.0);
        a_x.partial_cmp(&b_x).unwrap()
    });

    let mut lower_hull = points.iter()
        .fold(vec![], |hull, p| {
            grow_convex_hull(hull, p)
        });
    let mut upper_hull = points.iter().rev()
        .fold(vec![], |hull, p| {
            grow_convex_hull(hull, p)
        });

    //First and last element of both hull parts are the same point
    upper_hull.pop();
    lower_hull.pop();

    lower_hull.append(&mut upper_hull);
    lower_hull
}

fn grow_convex_hull(mut h: Vec<Point>, np: &Point) -> Vec<Point> {
    //pop all points from the hull which will be made irrelevant due to the new point
    while h.len() >= 2 && cross(&h[h.len() - 2], &h[h.len() - 1], &np) <= 0.0 {
        h.pop();
    }
    h.push(*np);
    h
}

fn cross(a: &Point, b: &Point, c: &Point) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}