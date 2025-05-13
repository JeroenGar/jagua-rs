#[cfg(feature = "separation-distance")]
use geo_offset::Offset;
use itertools::Itertools;
use log::{debug, info};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::SPolygon;

use anyhow::Result;

/// Whether to strictly inflate or deflate when making any modifications to shape.
/// Depends on the [`position`](crate::collision_detection::hazards::HazardEntity::position) of the [`HazardEntity`](crate::collision_detection::hazards::HazardEntity) that the shape represents.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeModifyMode {
    /// Modify the shape to be strictly larger than the original (superset).
    Inflate,
    /// Modify the shape to be strictly smaller than the original (subset).
    Deflate,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShapeModifyConfig {
    /// Maximum deviation of the simplified polygon with respect to the original polygon area as a ratio.
    /// If undefined, no simplification is performed.
    /// See [`simplify_shape`]
    pub simplify_tolerance: Option<f32>,
    /// Offset by which to inflate or deflate the polygon.
    /// If undefined, no offset is applied.
    /// See [`offset_shape`]
    pub offset: Option<f32>,
}

/// Simplifies a [`SPolygon`] by reducing the number of edges.
///
/// The simplified shape will either be a subset or a superset of the original shape, depending on the [`ShapeModifyMode`].
/// The procedure sequentially eliminates edges until either the change in area (ratio)
/// exceeds `max_area_delta` or the number of edges < 4.
pub fn simplify_shape(
    shape: &SPolygon,
    mode: ShapeModifyMode,
    max_area_change_ratio: f32,
) -> SPolygon {
    let original_area = shape.area;

    let mut ref_points = shape.vertices.clone();

    for _ in 0..shape.n_vertices() {
        let n_points = ref_points.len() as isize;
        if n_points < 4 {
            //can't simplify further
            break;
        }

        let mut corners = (0..n_points)
            .map(|i| {
                let i_prev = (i - 1).rem_euclid(n_points);
                let i_next = (i + 1).rem_euclid(n_points);
                Corner(i_prev as usize, i as usize, i_next as usize)
            })
            .collect_vec();

        if mode == ShapeModifyMode::Deflate {
            //default mode is to inflate, so we need to reverse the order of the corners and flip the corners for deflate mode
            //reverse the order of the corners
            corners.reverse();
            //reverse each corner
            corners.iter_mut().for_each(|c| c.flip());
        }

        let mut candidates = vec![];

        let mut prev_corner = corners.last().expect("corners is empty");
        let mut prev_corner_type = CornerType::from(prev_corner.to_points(&ref_points));

        //Go over all corners and generate candidates
        for corner in corners.iter() {
            let corner_type = CornerType::from(corner.to_points(&ref_points));

            //Generate a removal candidate (or not)
            match (&corner_type, &prev_corner_type) {
                (CornerType::Concave, _) => candidates.push(Candidate::Concave(*corner)),
                (CornerType::Collinear, _) => candidates.push(Candidate::Collinear(*corner)),
                (CornerType::Convex, CornerType::Convex) => {
                    candidates.push(Candidate::ConvexConvex(*prev_corner, *corner))
                }
                (_, _) => {}
            };
            (prev_corner, prev_corner_type) = (corner, corner_type);
        }

        //search the candidate with the smallest change in area that is valid
        let best_candidate = candidates
            .iter()
            .sorted_by_cached_key(|c| {
                calculate_area_delta(&ref_points, c)
                    .unwrap_or_else(|_| NotNan::new(f32::INFINITY).expect("area delta is NaN"))
            })
            .find(|c| candidate_is_valid(&ref_points, c));

        //if it is within the area change constraints, execute the candidate
        if let Some(best_candidate) = best_candidate {
            let new_shape = execute_candidate(&ref_points, best_candidate);
            let new_shape_area = SPolygon::calculate_area(&new_shape);
            let area_delta = (new_shape_area - original_area).abs() / original_area;
            if area_delta <= max_area_change_ratio {
                debug!(
                    "Simplified {:?} causing {:.2}% area change",
                    best_candidate,
                    area_delta * 100.0
                );
                ref_points = new_shape;
            } else {
                break; //area change too significant
            }
        } else {
            break; //no candidate found
        }
    }

    //Convert it back to a simple polygon
    let simpl_shape = SPolygon::new(ref_points).unwrap();

    if simpl_shape.n_vertices() < shape.n_vertices() {
        info!(
            "[PS] simplified from {} to {} edges with {:.3}% area difference",
            shape.n_vertices(),
            simpl_shape.n_vertices(),
            (simpl_shape.area - shape.area) / shape.area * 100.0
        );
    } else {
        info!("[PS] no simplification possible within area change constraints");
    }

    simpl_shape
}

fn calculate_area_delta(
    shape: &[Point],
    candidate: &Candidate,
) -> Result<NotNan<f32>, InvalidCandidate> {
    //calculate the difference in area of the shape if the candidate were to be executed
    let area = match candidate {
        Candidate::Collinear(_) => 0.0,
        Candidate::Concave(c) => {
            //Triangle formed by i_prev, i and i_next will correspond to the change area
            let Point(x0, y0) = shape[c.0];
            let Point(x1, y1) = shape[c.1];
            let Point(x2, y2) = shape[c.2];

            let area = (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1) / 2.0;

            area.abs()
        }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, (*c1, *c2))?;

            //the triangle formed by corner c1, c2, and replacing vertex will correspond to the change in area
            let Point(x0, y0) = shape[c1.1];
            let Point(x1, y1) = replacing_vertex;
            let Point(x2, y2) = shape[c2.1];

            let area = (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1) / 2.0;

            area.abs()
        }
    };
    Ok(NotNan::new(area).expect("area is NaN"))
}

fn candidate_is_valid(shape: &[Point], candidate: &Candidate) -> bool {
    //ensure the removal/replacement does not create any self intersections
    match candidate {
        Candidate::Collinear(_) => true,
        Candidate::Concave(c) => {
            let new_edge = Edge::try_new(shape[c.0], shape[c.2]).unwrap();
            let affected_points = [shape[c.0], shape[c.1], shape[c.2]];

            //check for self-intersections
            edge_iter(shape)
                .filter(|l| !affected_points.contains(&l.start))
                .filter(|l| !affected_points.contains(&l.end))
                .all(|l| !l.collides_with(&new_edge))
        }
        Candidate::ConvexConvex(c1, c2) => {
            match replacing_vertex_convex_convex_candidate(shape, (*c1, *c2)) {
                Err(_) => false,
                Ok(new_vertex) => {
                    let new_edge_1 = Edge::try_new(shape[c1.0], new_vertex).unwrap();
                    let new_edge_2 = Edge::try_new(new_vertex, shape[c2.2]).unwrap();

                    let affected_points = [shape[c1.1], shape[c1.0], shape[c2.1], shape[c2.2]];

                    //check for self-intersections
                    edge_iter(shape)
                        .filter(|l| !affected_points.contains(&l.start))
                        .filter(|l| !affected_points.contains(&l.end))
                        .all(|l| !l.collides_with(&new_edge_1) && !l.collides_with(&new_edge_2))
                }
            }
        }
    }
}

fn edge_iter(points: &[Point]) -> impl Iterator<Item = Edge> + '_ {
    let n_points = points.len();
    (0..n_points).map(move |i| {
        let j = (i + 1) % n_points;
        Edge::try_new(points[i], points[j]).unwrap()
    })
}

fn execute_candidate(shape: &[Point], candidate: &Candidate) -> Vec<Point> {
    let mut points = shape.iter().cloned().collect_vec();
    match candidate {
        Candidate::Collinear(c) | Candidate::Concave(c) => {
            points.remove(c.1);
        }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, (*c1, *c2))
                .expect("invalid candidate cannot be executed");
            points.remove(c1.1);
            let other_index = if c1.1 < c2.1 { c2.1 - 1 } else { c2.1 };
            points.remove(other_index);
            points.insert(other_index, replacing_vertex);
        }
    }
    points
}

fn replacing_vertex_convex_convex_candidate(
    shape: &[Point],
    (c1, c2): (Corner, Corner),
) -> Result<Point, InvalidCandidate> {
    assert_eq!(c1.2, c2.1, "non-consecutive corners {c1:?},{c2:?}");
    assert_eq!(c1.1, c2.0, "non-consecutive corners {c1:?},{c2:?}");

    let edge_prev = Edge::try_new(shape[c1.0], shape[c1.1]).unwrap();
    let edge_next = Edge::try_new(shape[c2.2], shape[c2.1]).unwrap();

    calculate_intersection_in_front(&edge_prev, &edge_next).ok_or(InvalidCandidate)
}

fn calculate_intersection_in_front(l1: &Edge, l2: &Edge) -> Option<Point> {
    //Calculates the intersection point between l1 and l2 if both were extended in front to infinity.

    //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    //vector 1 = [(x1,y1),(x2,y2)[ and vector 2 = [(x3,y3),(x4,y4)[
    let Point(x1, y1) = l1.start;
    let Point(x2, y2) = l1.end;
    let Point(x3, y3) = l2.start;
    let Point(x4, y4) = l2.end;

    //used formula is slightly different to the one on wikipedia. The orientation of the line segments are flipped
    //We consider an intersection if t == ]0,1] && u == ]0,1]

    let t_nom = (x2 - x4) * (y4 - y3) - (y2 - y4) * (x4 - x3);
    let t_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);

    let u_nom = (x2 - x4) * (y2 - y1) - (y2 - y4) * (x2 - x1);
    let u_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);

    let t = if t_denom != 0.0 { t_nom / t_denom } else { 0.0 };

    let u = if u_denom != 0.0 { u_nom / u_denom } else { 0.0 };

    if t < 0.0 && u < 0.0 {
        //intersection is in front both vectors
        Some(Point(x2 + t * (x1 - x2), y2 + t * (y1 - y2)))
    } else {
        //no intersection (parallel or not in front)
        None
    }
}

#[derive(Debug, Clone)]
struct InvalidCandidate;

#[derive(Clone, Debug, PartialEq)]
enum Candidate {
    Concave(Corner),
    ConvexConvex(Corner, Corner),
    Collinear(Corner),
}

#[derive(Clone, Copy, Debug, PartialEq)]
///Corner is defined as the left hand side of points 0-1-2
struct Corner(pub usize, pub usize, pub usize);

impl Corner {
    pub fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.2);
    }

    pub fn to_points(self, points: &[Point]) -> [Point; 3] {
        [points[self.0], points[self.1], points[self.2]]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CornerType {
    Concave,
    Convex,
    Collinear,
}

impl CornerType {
    pub fn from([p1, p2, p3]: [Point; 3]) -> Self {
        //returns the corner type on the left-hand side p1->p2->p3
        //From: https://algorithmtutor.com/Computational-Geometry/Determining-if-two-consecutive-segments-turn-left-or-right/

        let p1p2 = (p2.0 - p1.0, p2.1 - p1.1);
        let p1p3 = (p3.0 - p1.0, p3.1 - p1.1);
        let cross_prod = p1p2.0 * p1p3.1 - p1p2.1 * p1p3.0;

        //a positive cross product indicates that p2p3 turns to the left with respect to p1p2
        match cross_prod.partial_cmp(&0.0).expect("cross product is NaN") {
            Ordering::Less => CornerType::Concave,
            Ordering::Equal => CornerType::Collinear,
            Ordering::Greater => CornerType::Convex,
        }
    }
}

/// Offsets a [`SPolygon`] by a certain `distance` either inwards or outwards depending on the [`ShapeModifyMode`].
/// Relies on the [`geo_offset`](https://crates.io/crates/geo_offset) crate.
#[cfg(feature = "separation-distance")]
pub fn offset_shape(sp: &SPolygon, mode: ShapeModifyMode, distance: f32) -> Result<SPolygon> {
    let offset = match mode {
        ShapeModifyMode::Deflate => -distance,
        ShapeModifyMode::Inflate => distance,
    };

    // Convert the SPolygon to a geo_types::Polygon
    let geo_poly =
        geo_types::Polygon::new(sp.vertices.iter().map(|p| (p.0, p.1)).collect(), vec![]);

    // Create the offset geo_types::Polygon
    let geo_poly_offset = geo_poly
        .offset(offset)
        .map_err(|e| anyhow::anyhow!("Error while offsetting polygon: {:?}", e))?
        .0
        .remove(0);

    let mut points_offset = geo_poly_offset
        .exterior()
        .points()
        .map(|p| Point(p.x(), p.y()))
        .collect_vec();

    //pop the last point if it is the same as the first
    if points_offset.first() == points_offset.last() {
        points_offset.pop();
    }

    SPolygon::new(points_offset)
}

#[cfg(not(feature = "separation-distance"))]
pub fn offset_shape(_sp: &SPolygon, _mode: ShapeModifyMode, _distance: f32) -> Result<SPolygon> {
    anyhow::bail!(
        "cannot offset shape without geo_offset dependency, compile with --features separation to enable this"
    )
}
