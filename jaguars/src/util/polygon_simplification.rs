use std::cmp::Ordering;
use itertools::Itertools;
use log::{Level, log};
use serde::{Deserialize, Serialize};

use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PolySimplConfig {
    Disabled,
    Enabled {
        tolerance: f64, //max deviation from the original polygon area (in %)
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PolySimplMode {
    Inflate,
    Deflate,
}

impl PolySimplMode {
    pub fn flip(&self) -> Self {
        match self {
            Self::Inflate => Self::Deflate,
            Self::Deflate => Self::Inflate,
        }
    }
}


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

    pub fn to_points(&self, points: &[Point]) -> [Point;3] {
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
    pub fn from([p1,p2,p3]: [Point;3]) -> Self {
        //returns the corner type on the left-hand side p1->p2->p3
        //From: https://algorithmtutor.com/Computational-Geometry/Determining-if-two-consecutive-segments-turn-left-or-right/

        let p1p2 = (p2.0 - p1.0, p2.1 - p1.1);
        let p1p3 = (p3.0 - p1.0, p3.1 - p1.1);
        let cross_prod = p1p2.0 * p1p3.1 - p1p2.1 * p1p3.0;

        //a positive cross product indicates that p2p3 turns to the left with respect to p1p2
        match cross_prod.partial_cmp(&0.0).expect("cross product is NaN"){
            Ordering::Less => CornerType::Concave,
            Ordering::Equal => CornerType::Collinear,
            Ordering::Greater => CornerType::Convex,
        }
    }
}

//Simplifies a poly (removing vertices) until the area is inflated by a given factor or there are only 3 vertices left.
//SimplePolygons with positive area (counterclockwise vertices) will be fully enclosed by the simplified shape, while those with negative area (clockwise vertices) will fully enclose the simplified shape.
pub fn simplify_shape(shape: &SimplePolygon, mode: PolySimplMode, max_area_delta: f64) -> SimplePolygon {
    let original_area = shape.area();

    let mut ref_shape = shape.clone();

    for _ in 0..shape.number_of_points() {
        let mut candidates = vec![];

        if ref_shape.number_of_points() < 4 {
            //can't simplify further
            break;
        }

        let n_points = ref_shape.number_of_points() as isize;
        let mut corners = (0..n_points).map(|i| {
            let i_prev = (i - 1).rem_euclid(n_points);
            let i_next = (i + 1).rem_euclid(n_points);
            Corner(i_prev as usize, i as usize, i_next as usize)
        }).collect_vec();

        if mode == PolySimplMode::Deflate {
            //default mode is to inflate, so we need to reverse the order of the corners and flip the corners for deflate mode
            //reverse the order of the corners
            corners.reverse();
            //reverse each corner
            corners.iter_mut().for_each(|c| c.flip());

            //println!("corners: {:?}", corners)
        }

        let mut prev_corner = corners.last().expect("corners is empty");
        let mut prev_corner_type = CornerType::from(prev_corner.to_points(ref_shape.points()));
        for corner in corners.iter() {
            let corner_type = CornerType::from(corner.to_points(ref_shape.points()));

            //Generate a removal candidate (or not)
            let candidate = match (&corner_type, &prev_corner_type) {
                (CornerType::Concave, _) => {
                    Some(Candidate::Concave(*corner))
                }
                (CornerType::Collinear, _) => {
                    Some(Candidate::Collinear(*corner))
                }
                (CornerType::Convex, CornerType::Convex) => {
                    Some(Candidate::ConvexConvex(*prev_corner, *corner))
                }
                (_, _) => None,
            };
            if let Some(candidate) = candidate {
                if candidate_is_valid(&ref_shape, &candidate) {
                    candidates.push(candidate);
                }
            }
            prev_corner = corner;
            prev_corner_type = corner_type;
        }

        let best_candidate = candidates.iter().min_by(|a, b| {
            calculate_area_delta(&ref_shape, a).partial_cmp(&calculate_area_delta(&ref_shape, b)).unwrap()
        });


        if let Some(best_candidate) = best_candidate {
            let new_shape = execute_candidate(&ref_shape, best_candidate);
            let area_delta = (new_shape.area() - original_area).abs() / original_area;
            if area_delta <= max_area_delta {
                log!(Level::Debug, "removed vertex: {:.2}% area change ({:?})", area_delta * 100.0, best_candidate);
                ref_shape = new_shape;
            } else {
                break; //area change too significant
            }
        } else {
            break; //no candidate found
        }
    }

    log!(Level::Info, "simplified from {} to {} edges ({:.3}% area difference)", shape.number_of_points(), ref_shape.number_of_points(), (ref_shape.area() - shape.area()) / shape.area() * 100.0);

    return ref_shape;
}

fn calculate_area_delta(shape: &SimplePolygon, candidate: &Candidate) -> f64 {
    //calculate the difference in area of the shape if the candidate were to be executed
    match candidate {
        Candidate::Collinear(_) => {
            0.0
        }
        Candidate::Concave(c) => {

            //Triangle formed by i_prev, i and i_next will correspond to the change area
            let Point(x0, y0) = shape.get_point(c.0);
            let Point(x1, y1) = shape.get_point(c.1);
            let Point(x2, y2) = shape.get_point(c.2);

            let area = (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1) / 2.0;

            area.abs()
        }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, candidate).expect("candidate is not valid");

            //the triangle formed by corner c1, c2, and replacing vertex will correspond to the change in area
            let Point(x0, y0) = shape.get_point(c1.1);
            let Point(x1, y1) = replacing_vertex;
            let Point(x2, y2) = shape.get_point(c2.1);

            let area = (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1) / 2.0;

            area.abs()
        }
    }
}

fn candidate_is_valid(shape: &SimplePolygon, candidate: &Candidate) -> bool {
    //ensure the removal/replacement does not create any self intersections
    match candidate {
        Candidate::Collinear(_) => true,
        Candidate::Concave(c) => {
            let new_edge = Edge::new(shape.get_point(c.0), shape.get_point(c.2));
            let affected_points = [shape.get_point(c.0), shape.get_point(c.1), shape.get_point(c.2)];

            shape.edge_iter()
                .filter(|l| !affected_points.contains(&l.start()) && !affected_points.contains(&l.end())) //filter edges with points that are not affected by the candidate
                .all(|l| { !l.collides_with(&new_edge) })
        }
        Candidate::ConvexConvex(c1, c2) => {
            let new_vertex = replacing_vertex_convex_convex_candidate(shape, candidate);
            match new_vertex {
                None => false,
                Some(new_vertex) => {
                    let new_edge_1 = Edge::new(shape.get_point(c1.0), new_vertex);
                    let new_edge_2 = Edge::new(new_vertex, shape.get_point(c2.2));

                    let affected_points = [shape.get_point(c1.1), shape.get_point(c1.0), shape.get_point(c2.1), shape.get_point(c2.2)];

                    shape.edge_iter()
                        .filter(|l| !affected_points.contains(&l.start()) && !affected_points.contains(&l.end())) //filter edges with points that are not affected by the candidate
                        .all(|l| { !l.collides_with(&new_edge_1) && !l.collides_with(&new_edge_2) })
                }
            }
        }
    }
}

fn execute_candidate(shape: &SimplePolygon, candidate: &Candidate) -> SimplePolygon {
    let mut points = shape.points().clone();
    match candidate {
        Candidate::Collinear(c) => { points.remove(c.1); }
        Candidate::Concave(c) => { points.remove(c.1); }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, candidate).expect("candidate is not valid");
            points.remove(c1.1);
            let other_index = if c1.1 < c2.1 { c2.1 - 1 } else { c2.1 };
            points.remove(other_index);
            points.insert(other_index, replacing_vertex);
        }
    }
    let new_shape = SimplePolygon::new(points);
    new_shape
}

fn replacing_vertex_convex_convex_candidate(shape: &SimplePolygon, candidate: &Candidate) -> Option<Point> {
    match candidate {
        Candidate::ConvexConvex(c1, c2) => {
            assert!(c1.2 == c2.1 && c1.1 == c2.0, "non-consecutive corners {:?},{:?}", c1, c2); //ensure the corners are adjacent

            let edge_prev = Edge::new(shape.get_point(c1.0), shape.get_point(c1.1));
            let edge_next = Edge::new(shape.get_point(c2.2), shape.get_point(c2.1));

            calculate_intersection_in_front(&edge_prev, &edge_next)
        }
        _ => panic!("candidate is not of type convex convex")
    }
}

fn calculate_intersection_in_front(l1: &Edge, l2: &Edge) -> Option<Point> {
    //Calculates the intersection point between l1 and l2 if both were extended in front to infinity.

    //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    //vector 1 = [(x1,y1),(x2,y2)[ and vector 2 = [(x3,y3),(x4,y4)[
    let Point(x1, y1) = l1.start();
    let Point(x2, y2) = l1.end();
    let Point(x3, y3) = l2.start();
    let Point(x4, y4) = l2.end();

    //used formula is slightly different to the one on wikipedia. The orientation of the line segments are flipped
    //We consider an intersection if t == ]0,1] && u == ]0,1]

    let t_nom = (x2 - x4) * (y4 - y3) - (y2 - y4) * (x4 - x3);
    let t_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);

    let u_nom = (x2 - x4) * (y2 - y1) - (y2 - y4) * (x2 - x1);
    let u_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);

    let t;
    if t_denom != 0.0 {
        t = t_nom / t_denom;
    } else {
        t = 0.0;
    }
    let u;
    if u_denom != 0.0 {
        u = u_nom / u_denom;
    } else {
        u = 0.0;
    }

    if t < 0.0 && u < 0.0 {
        //intersection is in front both vectors
        Some(Point(x2 + t * (x1 - x2), y2 + t * (y1 - y2)))
    } else {
        //no intersection (parallel or not in front)
        None
    }
}