use std::collections::VecDeque;

use ordered_float::NotNan;

use crate::geometry::geo_traits::{CollidesWith, DistanceFrom, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::simple_polygon::SimplePolygon;


/// Generates the Pole of Inaccessibility (PoI)
/// The interior is defined as the interior of the shape minus the interior of the poles
/// Based on Mapbox's "Polylabel" algorithm: <https://github.com/mapbox/polylabel>
pub fn generate_next_pole(shape: &SimplePolygon, poles: &[Circle]) -> Circle {
    let square_bbox = shape.bbox().inflate_to_square();
    let root = POINode::new(square_bbox, MAX_POI_TREE_DEPTH, shape, &poles);
    let mut queue = VecDeque::from([root]);
    let mut best: Option<Circle> = None;
    let distance = |circle: &Option<Circle>| circle.as_ref().map_or(0.0, |c| c.radius);

    while let Some(node) = queue.pop_front() {
        //check if better than current best
        if node.distance > distance(&best) {
            best = Some(Circle::new(node.bbox.centroid(), node.distance));
        }

        //see if worth it to split
        if node.distance_upperbound() > distance(&best) {
            if let Some(children) = node.split(shape, &poles) {
                queue.extend(children);
            }
        }
    }
    best.expect("no pole present")
}

///Generates additional poles for a shape alongside the PoI
pub fn generate_additional_surrogate_poles(shape: &SimplePolygon, max_poles: usize, coverage_goal: f64) -> Vec<Circle> {
    let mut all_poles = vec![shape.poi().clone()];
    let pole_area_goal = shape.area() * coverage_goal;
    let mut total_pole_area = shape.poi().area();

    //Generate the poles
    for _ in 0..max_poles {
        let next = generate_next_pole(shape, &all_poles);

        total_pole_area += next.area();
        all_poles.push(next);

        if total_pole_area > pole_area_goal {
            //sufficient poles generated
            break;
        }
    }

    let mut surrogate_poles = all_poles[1..].to_vec();

    //TODO: test performance impact of sorting
    let sorted_poles = {
        let mut sorted_poles: Vec<Circle> = surrogate_poles.clone();

        while !surrogate_poles.is_empty() {
            let next_pole = surrogate_poles.iter().enumerate().max_by_key(|(_i, p)| {
                //or to centroid if no other poles present
                let min_distance_to_existing_poles = sorted_poles.iter()
                    .chain([shape.poi()])
                    .map(|p2| {
                        let d = p.distance_from_border(&p2.centroid()).1;
                        NotNan::new(d).unwrap()
                    })
                    .min().unwrap();

                let radius = p.radius;

                NotNan::new(radius.powi(2)).unwrap() * min_distance_to_existing_poles
            }).unwrap();
            sorted_poles.push(surrogate_poles.remove(next_pole.0));
        }

        sorted_poles
    };
    sorted_poles
}

pub const MAX_POI_TREE_DEPTH: usize = 10;

struct POINode {
    pub level: usize,
    pub bbox: AARectangle,
    pub radius: f64,
    pub distance: f64,
}

impl POINode {
    pub fn new(bbox: AARectangle, level: usize, poly: &SimplePolygon, poles: &[Circle]) -> Self {
        let radius = bbox.diameter() / 2.0;

        let centroid_inside = poly.collides_with(&bbox.centroid())
            && poles.iter().all(|c| !c.collides_with(&bbox.centroid()));

        let distance = {
            let distance_to_edges = poly.edge_iter()
                .map(|e| e.distance(&bbox.centroid()));

            let distance_to_poles = poles.iter()
                .map(|c| c.distance_from_border(&bbox.centroid()).1);

            let distance_to_border = distance_to_edges.chain(distance_to_poles)
                .fold(f64::MAX, |acc, d| acc.min(d));

            //if the centroid is outside, distance is counted negative
            match centroid_inside {
                true => distance_to_border,
                false => -distance_to_border,
            }
        };

        Self {
            bbox,
            level,
            radius,
            distance,
        }
    }

    pub fn split(&self, poly: &SimplePolygon, poles: &[Circle]) -> Option<[POINode; 4]> {
        match self.level {
            0 => None,
            _ => Some(
                self.bbox.quadrants()
                    .map(|qd| POINode::new(qd, self.level - 1, poly, poles))
            )
        }
    }

    pub fn distance_upperbound(&self) -> f64 {
        self.radius + self.distance
    }
}
