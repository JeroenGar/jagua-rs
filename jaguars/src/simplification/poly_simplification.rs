use itertools::Itertools;
use log::{Level, log};

use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::simplification::corner::{Corner, CornerType};
use crate::simplification::simplification_config::PolySimplMode;

//Simplifies a poly (removing vertices) until the area is inflated by a given factor or there are only 3 vertices left.
//SimplePolygons with positive area (counterclockwise vertices) will be fully enclosed by the simplified shape, while those with negative area (clockwise vertices) will fully enclose the simplified shape.
pub fn simplify_simple_poly(shape: &SimplePolygon, max_area_delta: f64, mode: PolySimplMode) -> SimplePolygon {
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
            Corner::new(i_prev as usize, i as usize, i_next as usize)
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
        let mut prev_corner_type = prev_corner.determine_type(ref_shape.points());
        for corner in corners.iter() {
            let corner_type = corner.determine_type(ref_shape.points());

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

#[derive(Clone, Debug, PartialEq)]
pub enum Candidate {
    Concave(Corner),
    ConvexConvex(Corner, Corner),
    Collinear(Corner),
}


pub fn calculate_area_delta(shape: &SimplePolygon, candidate: &Candidate) -> f64 {
    //calculate the difference in area of the shape if the candidate were to be executed
    match candidate {
        Candidate::Collinear(_) => {
            0.0
        }
        Candidate::Concave(c) => {

            //Triangle formed by i_prev, i and i_next will correspond to the change area
            let Point(x0, y0) = shape.get_point(c.i_prev());
            let Point(x1, y1) = shape.get_point(c.i());
            let Point(x2, y2) = shape.get_point(c.i_next());

            let area = (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1) / 2.0;

            area.abs()
        }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, candidate).expect("candidate is not valid");

            //the triangle formed by corner c1, c2, and replacing vertex will correspond to the change in area
            let Point(x0, y0) = shape.get_point(c1.i());
            let Point(x1, y1) = replacing_vertex;
            let Point(x2, y2) = shape.get_point(c2.i());

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
            let new_edge = Edge::new(shape.get_point(c.i_prev()), shape.get_point(c.i_next()));
            let affected_points = [shape.get_point(c.i_prev()), shape.get_point(c.i()), shape.get_point(c.i_next())];

            shape.edge_iter()
                .filter(|l| !affected_points.contains(&l.start()) && !affected_points.contains(&l.end())) //filter edges with points that are not affected by the candidate
                .all(|l| { !l.collides_with(&new_edge) })
        }
        Candidate::ConvexConvex(c1, c2) => {
            let new_vertex = replacing_vertex_convex_convex_candidate(shape, candidate);
            match new_vertex {
                None => false,
                Some(new_vertex) => {
                    let new_edge_1 = Edge::new(shape.get_point(c1.i_prev()), new_vertex);
                    let new_edge_2 = Edge::new(new_vertex, shape.get_point(c2.i_next()));

                    let affected_points = [shape.get_point(c1.i()), shape.get_point(c1.i_prev()), shape.get_point(c2.i()), shape.get_point(c2.i_next())];

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
        Candidate::Collinear(c) => { points.remove(c.i()); }
        Candidate::Concave(c) => { points.remove(c.i()); }
        Candidate::ConvexConvex(c1, c2) => {
            let replacing_vertex = replacing_vertex_convex_convex_candidate(shape, candidate).expect("candidate is not valid");
            points.remove(c1.i());
            let other_index = if c1.i() < c2.i() { c2.i() - 1 } else { c2.i() };
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
            assert!(c1.i_next() == c2.i() && c1.i() == c2.i_prev(), "non-consecutive corners {:?},{:?}", c1, c2); //ensure the corners are adjacent

            let edge_prev = Edge::new(shape.get_point(c1.i_prev()), shape.get_point(c1.i()));
            let edge_next = Edge::new(shape.get_point(c2.i_next()), shape.get_point(c2.i()));

            calculate_intersection_in_front(&edge_prev, &edge_next)
        }
        _ => panic!("candidate is not of type convex convex")
    }
}

fn get_points(shape: &SimplePolygon, corner: Corner) -> (Point, Point, Point) {
    (shape.get_point(corner.i_prev()), shape.get_point(corner.i()), shape.get_point(corner.i_next()))
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