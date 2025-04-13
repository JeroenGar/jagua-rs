use std::collections::VecDeque;

use ordered_float::NotNan;

use crate::fsize;
use crate::geometry::geo_traits::{CollidesWith, Distance, SeparationDistance, Shape};
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::SimplePolygon;

/// Generates the Pole of Inaccessibility (PoI). The PoI is the point in the interior of the shape that is farthest from the boundary.
/// The interior is defined as the interior of the `shape` minus the interior of the `poles`.
pub fn generate_next_pole(shape: &SimplePolygon, poles: &[Circle]) -> Circle {
    //Based on Mapbox's "Polylabel" algorithm: <https://github.com/mapbox/polylabel>
    let square_bbox = shape.bbox().inflate_to_square();
    let root = POINode::new(square_bbox, MAX_POI_TREE_DEPTH, shape, poles);
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
            if let Some(children) = node.split(shape, poles) {
                queue.extend(children);
            }
        }
    }
    best.expect("no pole present")
}

///Generates additional poles for a shape alongside the PoI
pub fn generate_poles(shape: &SimplePolygon, n_pole_limits: &[(usize, fsize)]) -> Vec<Circle> {
    //generate the additional poles
    let additional_poles = {
        let mut all_poles = vec![shape.poi.clone()];
        let mut total_pole_area = shape.poi.area();

        //Generate the poles
        loop {
            let next = generate_next_pole(shape, &all_poles);

            total_pole_area += next.area();
            all_poles.push(next);

            let current_coverage = total_pole_area / shape.area();

            //check if any limit in the number of poles is reached at this coverage
            let active_pole_limit = n_pole_limits
                .iter()
                .filter(|(_, coverage_threshold)| current_coverage > *coverage_threshold)
                .min_by_key(|(n_poles, _)| *n_poles)
                .map(|(n_poles, _)| n_poles);

            if let Some(active_pole_limit) = active_pole_limit {
                if all_poles.len() >= *active_pole_limit {
                    //stop generating if we are above the limit
                    break;
                }
            }
            assert!(
                all_poles.len() < 1000,
                "More than 1000 poles were generated, please check the SPSurrogateConfig"
            )
        }
        all_poles[1..].to_vec()
    };

    //Sort the poles to maximize chance of early fail fast
    let mut unsorted_poles = additional_poles.clone();
    let mut sorted_poles = vec![];

    while !unsorted_poles.is_empty() {
        //find the pole that maximizes the distance to the closest prior pole and is also large
        let (next_pole_index, _) = unsorted_poles
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let prior_poles = sorted_poles.iter().chain([&shape.poi]);

                let min_distance_prior_poles = prior_poles
                    .map(|prior| prior.separation_distance(&p.centroid()).1)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                (i, p.radius.powi(2) * min_distance_prior_poles)
            })
            .max_by_key(|(_i, obj)| NotNan::new(*obj).unwrap())
            .unwrap();
        sorted_poles.push(unsorted_poles.remove(next_pole_index));
    }

    sorted_poles
}

const MAX_POI_TREE_DEPTH: usize = 10;

struct POINode {
    pub level: usize,
    pub bbox: AARectangle,
    pub radius: fsize,
    pub distance: fsize,
}

impl POINode {
    fn new(bbox: AARectangle, level: usize, poly: &SimplePolygon, poles: &[Circle]) -> Self {
        let radius = bbox.diameter() / 2.0;

        let centroid_inside = poly.collides_with(&bbox.centroid())
            && poles.iter().all(|c| !c.collides_with(&bbox.centroid()));

        let distance = {
            let distance_to_edges = poly.edge_iter().map(|e| e.distance(&bbox.centroid()));

            let distance_to_poles = poles
                .iter()
                .map(|c| c.separation_distance(&bbox.centroid()).1);

            let distance_to_border = distance_to_edges
                .chain(distance_to_poles)
                .fold(fsize::MAX, |acc, d| acc.min(d));

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

    fn split(&self, poly: &SimplePolygon, poles: &[Circle]) -> Option<[POINode; 4]> {
        match self.level {
            0 => None,
            _ => Some(
                self.bbox
                    .quadrants()
                    .map(|qd| POINode::new(qd, self.level - 1, poly, poles)),
            ),
        }
    }

    fn distance_upperbound(&self) -> fsize {
        self.radius + self.distance
    }
}
