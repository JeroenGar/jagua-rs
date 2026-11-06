use std::collections::VecDeque;

use crate::geometry::geo_traits::{CollidesWith, DistanceTo, SeparationDistance};
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::SPolygon;

use anyhow::{Result, anyhow};
use log::debug;

/// Generates a set of non-overlapping poles for a shape according to specified coverage limits.
///
/// Each new pole is the largest one that fits outside the earlier poles, so the returned order is
/// largest-remaining-first. See [`compute_pole`] for details on what a pole is.
/// Generation also stops when no further pole is found, so a thin shape can have fewer poles than the limits allow.
#[must_use]
pub fn generate_surrogate_poles(shape: &SPolygon, n_pole_limits: &[(usize, f32)]) -> Vec<Circle> {
    let mut all_poles = vec![shape.poi];
    let mut total_pole_area = shape.poi.area();

    //Generate the poles until one of the pole number / coverage limits is reached
    loop {
        let current_coverage = total_pole_area / shape.area;

        //check if any limit in the number of poles is reached at this coverage
        let active_pole_limit = n_pole_limits
            .iter()
            .filter(|(_, coverage_threshold)| current_coverage > *coverage_threshold)
            .min_by_key(|(n_poles, _)| *n_poles)
            .map(|(n_poles, _)| n_poles);

        if let Some(active_pole_limit) = active_pole_limit
            && all_poles.len() >= *active_pole_limit
        {
            //stop generating if we are above the limit
            break;
        }

        //Unlike the pole of inaccessibility, these poles are optional: they are only searched at the default depth,
        //because searching deeper for each of them is very slow in thin shapes.
        let Some(next) = search_pole(shape, &all_poles, POI_TREE_DEPTHS[0]) else {
            //in a thin shape, the search can run out of room for poles before the limits are reached
            debug!("[POLE] no further pole found after {}", all_poles.len());
            break;
        };
        total_pole_area += next.area();
        all_poles.push(next);
        assert!(
            all_poles.len() < 1000,
            "More than 1000 poles were generated, please check the SPSurrogateConfig"
        );
    }
    all_poles
}

/// Computes the *pole* - the largest circle which is both inside of `shape` while being outside all other `poles`.
/// Closely related to [Pole of Inaccessibility (PoI)](https://en.wikipedia.org/wiki/Pole_of_inaccessibility),
/// and inspired by Mapbox's [`polylabel`](https://github.com/mapbox/polylabel) algorithm.
///
/// A thin shape can lie between the centers of all nodes of a shallow search tree,
/// so the search is repeated deeper until a pole is found.
/// Normal shapes find their pole at the first depth and pay nothing extra.
pub fn compute_pole(shape: &SPolygon, poles: &[Circle]) -> Result<Circle> {
    POI_TREE_DEPTHS
        .iter()
        .find_map(|&depth| search_pole(shape, poles, depth))
        .ok_or(anyhow!(
            "no pole found with up to {} levels of recursion. Please check the input shape: {:?}",
            POI_TREE_DEPTHS[POI_TREE_DEPTHS.len() - 1],
            shape.vertices
        ))
}

/// Depths of the successive pole searches in [`compute_pole`].
/// Each level halves the size of the nodes, the root being the square around the shape's bounding box.
/// Beyond about 24 levels, `f32` cannot represent the nodes' midpoints anymore.
const POI_TREE_DEPTHS: [usize; 4] = [10, 14, 18, 22];

fn search_pole(shape: &SPolygon, poles: &[Circle], depth: usize) -> Option<Circle> {
    let square_bbox = shape.bbox.inflate_to_square();
    let root = POINode::new(square_bbox, depth, shape, poles);
    let mut queue = VecDeque::from([root]);
    let mut best: Option<Circle> = None;
    let distance = |circle: &Option<Circle>| circle.as_ref().map_or(0.0, |c| c.radius);

    while let Some(node) = queue.pop_front() {
        //check if better than current best
        if node.distance > distance(&best) {
            best = Some(Circle::try_new(node.bbox.centroid(), node.distance).unwrap());
        }

        //see if worth it to split
        if node.distance_upperbound() > distance(&best)
            && let Some(children) = node.split(shape, poles)
        {
            queue.extend(children);
        }
    }
    best
}

struct POINode {
    pub level: usize,
    pub bbox: Rect,
    pub radius: f32,
    pub distance: f32,
}

impl POINode {
    fn new(bbox: Rect, level: usize, poly: &SPolygon, poles: &[Circle]) -> Self {
        let radius = bbox.diameter() / 2.0;

        let centroid_inside = poly.collides_with(&bbox.centroid())
            && poles.iter().all(|c| !c.collides_with(&bbox.centroid()));

        let distance = {
            let distance_to_edges = poly.edge_iter().map(|e| e.distance_to(&bbox.centroid()));

            let distance_to_poles = poles
                .iter()
                .map(|c| c.separation_distance(&bbox.centroid()).1);

            let distance_to_border = distance_to_edges
                .chain(distance_to_poles)
                .fold(f32::MAX, f32::min);

            //if the centroid is outside, distance is counted negative
            if centroid_inside {
                distance_to_border
            } else {
                -distance_to_border
            }
        };

        Self {
            level,
            bbox,
            radius,
            distance,
        }
    }

    fn split(&self, poly: &SPolygon, poles: &[Circle]) -> Option<[POINode; 4]> {
        let Point(m_x, m_y) = self.bbox.centroid();
        let Rect {
            x_min,
            y_min,
            x_max,
            y_max,
        } = self.bbox;
        //far from the origin, the midpoint can round onto a side, which would leave a quadrant without area
        let splittable = x_min < m_x && m_x < x_max && y_min < m_y && m_y < y_max;
        match self.level {
            0 => None,
            _ if !splittable => None,
            _ => Some(
                self.bbox
                    .quadrants()
                    .map(|qd| POINode::new(qd, self.level - 1, poly, poles)),
            ),
        }
    }

    fn distance_upperbound(&self) -> f32 {
        self.radius + self.distance
    }
}
