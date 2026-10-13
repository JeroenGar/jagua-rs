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
    /// Bounding boxes of each hazard's edges
    edges: Vec<EdgeBoxes>,
}

impl HazardIndex {
    pub fn insert(&mut self, hkey: HazKey, shape: &SPolygon) {
        let bbox = shape.bbox;
        self.edges.push(EdgeBoxes::new(shape));
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
        self.edges.swap_remove(i);
        self.x_min.swap_remove(i);
        self.y_min.swap_remove(i);
        self.x_max.swap_remove(i);
        self.y_max.swap_remove(i);
    }

    /// Calls `f` for every hazard whose bounding box overlaps `r` (borders included),
    /// until `f` returns `true`. Returns whether it did.
    #[inline(always)]
    pub fn any_overlapping(&self, r: Rect, mut f: impl FnMut(HazKey, &EdgeBoxes) -> bool) -> bool {
        for start in (0..self.keys.len()).step_by(64) {
            let mut mask = self.chunk_mask(start, r);
            while mask != 0 {
                let i = start + mask.trailing_zeros() as usize;
                mask &= mask - 1;
                if f(self.keys[i], &self.edges[i]) {
                    return true;
                }
            }
        }
        false
    }

    /// Like [`Self::any_overlapping`] for several rectangles at once, all contained in `union`.
    /// The arrays are scanned once with `union`; the few candidates are then tested per rectangle.
    #[inline(always)]
    pub fn any_overlapping_each<'a>(
        &self,
        union: Rect,
        rects: impl Iterator<Item = Rect> + Clone,
        mut f: impl FnMut(usize, HazKey, &EdgeBoxes) -> bool,
    ) -> bool {
        for start in (0..self.keys.len()).step_by(64) {
            let candidates = self.chunk_mask(start, union);
            if candidates == 0 {
                continue;
            }
            for (ri, r) in rects.clone().enumerate() {
                let mut mask = candidates;
                while mask != 0 {
                    let i = start + mask.trailing_zeros() as usize;
                    mask &= mask - 1;
                    let overlaps = (self.x_min[i] <= r.x_max)
                        & (self.x_max[i] >= r.x_min)
                        & (self.y_min[i] <= r.y_max)
                        & (self.y_max[i] >= r.y_min);
                    if overlaps && f(ri, self.keys[i], &self.edges[i]) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Bit `j` is set if the hazard at `start + j` overlaps `r`, for up to 64 hazards.
    #[inline(always)]
    fn chunk_mask(&self, start: usize, r: Rect) -> u64 {
        let end = (start + 64).min(self.keys.len());
        rect_mask(
            &self.x_min[start..end],
            &self.y_min[start..end],
            &self.x_max[start..end],
            &self.y_max[start..end],
            r,
        )
    }
}

/// Bounding boxes of a polygon's edges in flat arrays.
/// Edge `i` runs from vertex `i - 1` (the last vertex for `i = 0`) to vertex `i`.
#[derive(Clone, Debug, Default)]
pub struct EdgeBoxes {
    x_min: Vec<f32>,
    y_min: Vec<f32>,
    x_max: Vec<f32>,
    y_max: Vec<f32>,
}

impl EdgeBoxes {
    pub fn new(poly: &SPolygon) -> Self {
        let mut boxes = Self::default();
        let vertices = poly.vertices();
        let mut prev = *vertices.last().unwrap();
        for &cur in vertices {
            boxes.x_min.push(prev.0.min(cur.0));
            boxes.y_min.push(prev.1.min(cur.1));
            boxes.x_max.push(prev.0.max(cur.0));
            boxes.y_max.push(prev.1.max(cur.1));
            prev = cur;
        }
        boxes
    }

    /// Calls `f` with the index of every edge whose bounding box overlaps `r`, until it returns `true`.
    #[inline(always)]
    fn any_overlapping(&self, r: Rect, mut f: impl FnMut(usize) -> bool) -> bool {
        for start in (0..self.x_min.len()).step_by(64) {
            let end = (start + 64).min(self.x_min.len());
            let mut mask = rect_mask(
                &self.x_min[start..end],
                &self.y_min[start..end],
                &self.x_max[start..end],
                &self.y_max[start..end],
                r,
            );
            while mask != 0 {
                let i = start + mask.trailing_zeros() as usize;
                mask &= mask - 1;
                if f(i) {
                    return true;
                }
            }
        }
        false
    }
}

/// Bit `j` is set if box `j` overlaps `r` (borders included), for up to 64 boxes.
#[inline(always)]
fn rect_mask(x_min: &[f32], y_min: &[f32], x_max: &[f32], y_max: &[f32], r: Rect) -> u64 {
    let mut mask = 0u64;
    for (j, (((&x0, &y0), &x1), &y1)) in x_min.iter().zip(y_min).zip(x_max).zip(y_max).enumerate() {
        let overlaps = (x0 <= r.x_max) & (x1 >= r.x_min) & (y0 <= r.y_max) & (y1 >= r.y_min);
        mask |= u64::from(overlaps) << j;
    }
    mask
}

/// The edge of `vertices` ending at vertex `i`.
#[inline(always)]
fn edge_to(vertices: &[Point], i: usize) -> Edge {
    let prev = if i == 0 { vertices.len() - 1 } else { i - 1 };
    Edge {
        start: vertices[prev],
        end: vertices[i],
    }
}

/// Like [`edges_intersect`], with the bounding boxes of `b`'s edges given.
pub fn edges_intersect_cached(a: &SPolygon, b: &SPolygon, b_boxes: &EdgeBoxes, r: Rect) -> bool {
    let empty = Edge {
        start: Point(0.0, 0.0),
        end: Point(0.0, 0.0),
    };
    let mut buf_a = [empty; BUF];
    let Some(n_a) = edges_overlapping(a, &r, &mut buf_a) else {
        return edges_intersect(a, b, r);
    };
    if n_a == 0 {
        return false;
    }
    let a_edges = &buf_a[..n_a];
    let vb = b.vertices();
    b_boxes.any_overlapping(r, |i| {
        let eb = edge_to(vb, i);
        a_edges.iter().any(|ea| ea.collides_with(&eb))
    })
}

/// Like [`circle_touches_edges`], with the bounding boxes of `poly`'s edges given.
pub fn circle_touches_edges_cached(c: &Circle, poly: &SPolygon, boxes: &EdgeBoxes) -> bool {
    let sq_radius = c.radius.powi(2);
    let vertices = poly.vertices();
    boxes.any_overlapping(c.bbox(), |i| {
        edge_to(vertices, i).sq_distance_to(&c.center) <= sq_radius
    })
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

/// Cohen-Sutherland outcode of `p` relative to `r`: one bit per side of `r` that `p` lies beyond.
/// An edge can only overlap `r` if the outcodes of its endpoints share no bit.
#[inline(always)]
fn outcode(p: Point, r: &Rect) -> u8 {
    u8::from(p.0 < r.x_min)
        | (u8::from(p.0 > r.x_max) << 1)
        | (u8::from(p.1 < r.y_min) << 2)
        | (u8::from(p.1 > r.y_max) << 3)
}

const BUF: usize = 64;

/// Collects the edges of `poly` whose bounding box overlaps `r` into `buf`.
/// Returns `None` if they do not fit.
#[inline(always)]
fn edges_overlapping(poly: &SPolygon, r: &Rect, buf: &mut [Edge; BUF]) -> Option<usize> {
    let vertices = poly.vertices();
    let mut prev = *vertices.last().unwrap();
    let mut prev_code = outcode(prev, r);
    let mut n = 0;
    for &cur in vertices {
        let code = outcode(cur, r);
        if prev_code & code == 0 {
            if n == BUF {
                return None;
            }
            buf[n] = Edge {
                start: prev,
                end: cur,
            };
            n += 1;
        }
        (prev, prev_code) = (cur, code);
    }
    Some(n)
}

/// Whether any edge of `a` intersects any edge of `b`.
/// Only edges whose bounding box overlaps `r`, the overlap of both polygons' bounding boxes, can intersect.
pub fn edges_intersect(a: &SPolygon, b: &SPolygon, r: Rect) -> bool {
    let empty = Edge {
        start: Point(0.0, 0.0),
        end: Point(0.0, 0.0),
    };
    let (mut buf_a, mut buf_b) = ([empty; BUF], [empty; BUF]);
    match (
        edges_overlapping(b, &r, &mut buf_b),
        edges_overlapping(a, &r, &mut buf_a),
    ) {
        (Some(n_b), Some(n_a)) => buf_a[..n_a]
            .iter()
            .any(|ea| buf_b[..n_b].iter().any(|eb| ea.collides_with(eb))),
        // Too many candidate edges for the buffers: test all pairs directly
        _ => a.edge_iter().any(|ea| b.edge_iter().any(|eb| ea.collides_with(&eb))),
    }
}

/// Whether `c` touches any edge of `poly` whose bounding box overlaps the circle's bounding box.
pub fn circle_touches_edges(c: &Circle, poly: &SPolygon) -> bool {
    let r = c.bbox();
    let sq_radius = c.radius.powi(2);
    let vertices = poly.vertices();
    let mut prev = *vertices.last().unwrap();
    let mut prev_code = outcode(prev, &r);
    for &cur in vertices {
        let code = outcode(cur, &r);
        if prev_code & code == 0
            && (Edge {
                start: prev,
                end: cur,
            })
            .sq_distance_to(&c.center)
                <= sq_radius
        {
            return true;
        }
        (prev, prev_code) = (cur, code);
    }
    false
}
