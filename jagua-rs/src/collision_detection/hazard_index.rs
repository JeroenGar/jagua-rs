use crate::collision_detection::hazards::HazKey;
use crate::geometry::geo_traits::{CollidesWith, DistanceTo};
use crate::geometry::primitives::{Circle, Edge, Point, Rect, SPolygon};

/// Prototype of a hazard-first broad phase: the bounding boxes of all hazards in flat arrays.
#[derive(Clone, Debug, Default)]
pub struct HazardIndex {
    keys: Vec<HazKey>,
    x_min: Vec<f32>,
    y_min: Vec<f32>,
    x_max: Vec<f32>,
    y_max: Vec<f32>,
}

impl HazardIndex {
    pub fn insert(&mut self, hkey: HazKey, bbox: Rect) {
        self.keys.push(hkey);
        self.x_min.push(bbox.x_min);
        self.y_min.push(bbox.y_min);
        self.x_max.push(bbox.x_max);
        self.y_max.push(bbox.y_max);
    }

    pub fn remove(&mut self, hkey: HazKey) {
        let i = self
            .keys
            .iter()
            .position(|k| *k == hkey)
            .expect("hazard not in index");
        self.keys.swap_remove(i);
        self.x_min.swap_remove(i);
        self.y_min.swap_remove(i);
        self.x_max.swap_remove(i);
        self.y_max.swap_remove(i);
    }

    /// Calls `f` for every hazard whose bounding box overlaps `r` (borders included),
    /// until `f` returns `true`. Returns whether it did.
    #[inline(always)]
    pub fn any_overlapping(&self, r: Rect, mut f: impl FnMut(HazKey) -> bool) -> bool {
        for i in 0..self.keys.len() {
            let overlaps = (self.x_min[i] <= r.x_max)
                & (self.x_max[i] >= r.x_min)
                & (self.y_min[i] <= r.y_max)
                & (self.y_max[i] >= r.y_min);
            if overlaps && f(self.keys[i]) {
                return true;
            }
        }
        false
    }
}

/// Intersection of two rectangles, or `None` if they are disjoint (touching counts as overlap).
#[inline(always)]
pub fn overlap(a: Rect, b: Rect) -> Option<Rect> {
    let x_min = a.x_min.max(b.x_min);
    let y_min = a.y_min.max(b.y_min);
    let x_max = a.x_max.min(b.x_max);
    let y_max = a.y_max.min(b.y_max);
    (x_min <= x_max && y_min <= y_max).then_some(Rect {
        x_min,
        y_min,
        x_max,
        y_max,
    })
}

#[inline(always)]
fn edge_bbox_overlaps(e: &Edge, r: &Rect) -> bool {
    e.x_min().max(r.x_min) <= e.x_max().min(r.x_max)
        && e.y_min().max(r.y_min) <= e.y_max().min(r.y_max)
}

/// Whether any edge of `a` intersects any edge of `b`.
/// Only edges whose bounding box overlaps `r`, the overlap of both polygons' bounding boxes, can intersect.
pub fn edges_intersect(a: &SPolygon, b: &SPolygon, r: Rect) -> bool {
    const BUF: usize = 64;
    let mut buf = [Edge {
        start: Point(0.0, 0.0),
        end: Point(0.0, 0.0),
    }; BUF];
    let mut n = 0;
    for e in b.edge_iter() {
        if edge_bbox_overlaps(&e, &r) {
            if n == BUF {
                // Too many candidate edges for the buffer: test all pairs directly
                return a.edge_iter().filter(|ea| edge_bbox_overlaps(ea, &r)).any(|ea| {
                    b.edge_iter()
                        .filter(|eb| edge_bbox_overlaps(eb, &r))
                        .any(|eb| ea.collides_with(&eb))
                });
            }
            buf[n] = e;
            n += 1;
        }
    }
    if n == 0 {
        return false;
    }
    let b_edges = &buf[..n];
    a.edge_iter()
        .filter(|ea| edge_bbox_overlaps(ea, &r))
        .any(|ea| b_edges.iter().any(|eb| ea.collides_with(eb)))
}

/// Whether `c` touches any edge of `poly` whose bounding box overlaps the circle's bounding box.
pub fn circle_touches_edges(c: &Circle, poly: &SPolygon) -> bool {
    let r = c.bbox();
    let sq_radius = c.radius.powi(2);
    poly.edge_iter()
        .filter(|e| edge_bbox_overlaps(e, &r))
        .any(|e| e.sq_distance_to(&c.center) <= sq_radius)
}
