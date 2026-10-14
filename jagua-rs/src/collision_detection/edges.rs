// The query loops rely on being inlined into their callers
#![allow(clippy::inline_always)]

use crate::geometry::geo_traits::{CollidesWith, DistanceTo};
use crate::geometry::primitives::{Circle, Edge, Point, Rect, SPolygon};
use std::sync::Arc;

/// Bounding boxes of a polygon's edges in flat arrays, so they can be culled 64 at a time.
/// Box `i` bounds [`SPolygon::edge`]`(i)`. Cloning shares the boxes.
#[derive(Clone, Debug)]
pub(crate) struct EdgeBoxes {
    /// The `x_min`, `y_min`, `x_max` and `y_max` of all boxes, one array after the other
    bounds: Arc<[f32]>,
}

impl EdgeBoxes {
    pub fn new(poly: &SPolygon) -> Self {
        let bounds = [Edge::x_min, Edge::y_min, Edge::x_max, Edge::y_max]
            .iter()
            .flat_map(|bound| poly.edge_iter().map(move |e| bound(&e)))
            .collect();
        Self { bounds }
    }

    /// Calls `f` with every edge of `poly` whose bounding box overlaps `r`, until it returns `true`.
    /// Returns whether it did.
    #[inline(always)]
    pub fn any_edge_overlapping(
        &self,
        poly: &SPolygon,
        r: Rect,
        mut f: impl FnMut(&Edge) -> bool,
    ) -> bool {
        let vertices = poly.vertices();
        let n = vertices.len();
        debug_assert_eq!(self.bounds.len(), 4 * n);
        let (x_min, rest) = self.bounds.split_at(n);
        let (y_min, rest) = rest.split_at(n);
        let (x_max, y_max) = rest.split_at(n);
        for start in (0..n).step_by(64) {
            let end = (start + 64).min(n);
            let mut mask = rect_mask(
                &x_min[start..end],
                &y_min[start..end],
                &x_max[start..end],
                &y_max[start..end],
                r,
            );
            while mask != 0 {
                let i = start + mask.trailing_zeros() as usize;
                mask &= mask - 1;
                let edge = Edge {
                    start: vertices[i],
                    end: vertices[if i + 1 == n { 0 } else { i + 1 }],
                };
                if f(&edge) {
                    return true;
                }
            }
        }
        false
    }
}

/// A polygon's edges grouped in chains that run monotonically in y, each with a bounding box.
/// A horizontal ray crosses a y-monotone chain at most once, so a point-in-polygon test only
/// inspects the edges of chains whose box contains the point.
#[derive(Clone, Debug)]
pub(crate) struct EdgeChains {
    /// All edges, chain after chain
    edges: Vec<Edge>,
    chains: Vec<Chain>,
}

#[derive(Clone, Debug)]
struct Chain {
    /// The chain's edges in [`EdgeChains::edges`]
    range: std::ops::Range<usize>,
    bbox: Rect,
    /// y of the chain's first and last vertex
    y_ends: (f32, f32),
}

/// Longest chain, so diagonal runs keep small boxes
const MAX_CHAIN_EDGES: usize = 16;

impl EdgeChains {
    pub fn new(poly: &SPolygon) -> Self {
        let edges = poly.edge_iter().collect::<Vec<_>>();
        let mut chains: Vec<Chain> = vec![];
        // Direction in y of the last chain: 1 up, -1 down, 0 still undecided
        let mut dir = 0;
        for (i, e) in edges.iter().enumerate() {
            let d = match e.end.1.total_cmp(&e.start.1) {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
            };
            match chains.last_mut() {
                Some(c) if c.range.len() < MAX_CHAIN_EDGES && (d == 0 || dir == 0 || d == dir) => {
                    c.range.end = i + 1;
                    c.bbox = Rect::bounding_rect(c.bbox, e.bbox());
                    c.y_ends.1 = e.end.1;
                    if dir == 0 {
                        dir = d;
                    }
                }
                _ => {
                    chains.push(Chain {
                        range: i..i + 1,
                        bbox: e.bbox(),
                        y_ends: (e.start.1, e.end.1),
                    });
                    dir = d;
                }
            }
        }
        Self { edges, chains }
    }

    /// Calls `f` with every edge whose bounding box overlaps `r`, until it returns `true`.
    /// Returns whether it did.
    #[inline(always)]
    pub fn any_edge_overlapping(&self, r: Rect, mut f: impl FnMut(&Edge) -> bool) -> bool {
        for c in &self.chains {
            if !c.bbox.collides_with(&r) {
                continue;
            }
            for e in &self.edges[c.range.clone()] {
                if e.bbox().collides_with(&r) && f(e) {
                    return true;
                }
            }
        }
        false
    }

    /// Even-odd point-in-polygon test with a ray towards +x. Chains entirely right of the point
    /// count from their end points alone; only chains whose box contains the point test edges.
    pub fn contains(&self, p: Point) -> bool {
        let Point(px, py) = p;
        let mut inside = false;
        for c in &self.chains {
            if py < c.bbox.y_min || py > c.bbox.y_max || px > c.bbox.x_max {
                continue;
            }
            if px < c.bbox.x_min {
                // The ray crosses the chain once if its ends straddle the ray
                if (c.y_ends.0 > py) != (c.y_ends.1 > py) {
                    inside = !inside;
                }
                continue;
            }
            for e in &self.edges[c.range.clone()] {
                let (a, b) = (e.start, e.end);
                if (a.1 > py) != (b.1 > py) && px < (b.0 - a.0) * (py - a.1) / (b.1 - a.1) + a.0 {
                    inside = !inside;
                }
            }
        }
        inside
    }
}

/// A set of edges that can be culled by bounding box.
pub(crate) trait EdgeSet {
    /// Calls `f` with every edge whose bounding box overlaps `r`, until it returns `true`.
    /// Returns whether it did.
    fn any_edge_overlapping(&self, r: Rect, f: impl FnMut(&Edge) -> bool) -> bool;
}

impl EdgeSet for EdgeChains {
    #[inline(always)]
    fn any_edge_overlapping(&self, r: Rect, f: impl FnMut(&Edge) -> bool) -> bool {
        EdgeChains::any_edge_overlapping(self, r, f)
    }
}

/// Whether an edge of `poly` intersects an edge of `edges`.
/// Only edges whose bounding box overlaps `r` are considered on both sides.
#[inline(always)]
pub fn edges_intersect(poly: &SPolygon, r: Rect, edges: &impl EdgeSet) -> bool {
    // Edges of `poly` overlapping `r`, tested against the other side 64 at a time
    let mut buf = [Edge {
        start: Point(0.0, 0.0),
        end: Point(0.0, 0.0),
    }; 64];
    let mut n = 0;
    let vertices = poly.vertices();
    let mut prev = *vertices.last().unwrap();
    let mut prev_code = outcode(prev, r);
    for &cur in vertices {
        let code = outcode(cur, r);
        // An edge can only overlap `r` if its end points are not beyond the same side
        if prev_code & code == 0 {
            buf[n] = Edge {
                start: prev,
                end: cur,
            };
            n += 1;
            if n == buf.len() {
                if edges.any_edge_overlapping(r, |e| buf.iter().any(|b| b.collides_with(e))) {
                    return true;
                }
                n = 0;
            }
        }
        (prev, prev_code) = (cur, code);
    }
    n > 0 && edges.any_edge_overlapping(r, |e| buf[..n].iter().any(|b| b.collides_with(e)))
}

/// Whether `c` touches an edge of `edges`.
#[inline(always)]
pub fn circle_touches_edges(c: &Circle, edges: &impl EdgeSet) -> bool {
    let sq_radius = c.radius.powi(2);
    edges.any_edge_overlapping(c.bbox(), |e| e.sq_distance_to(&c.center) <= sq_radius)
}

/// Whether `edge` intersects an edge of `edges`.
#[inline(always)]
pub fn edge_touches_edges(edge: &Edge, edges: &impl EdgeSet) -> bool {
    edges.any_edge_overlapping(edge.bbox(), |e| e.collides_with(edge))
}

/// Intersection of two rectangles, or `None` if they are disjoint. Touching counts as overlap.
#[inline(always)]
pub fn overlap(a: Rect, b: Rect) -> Option<Rect> {
    let r = Rect {
        x_min: a.x_min.max(b.x_min),
        y_min: a.y_min.max(b.y_min),
        x_max: a.x_max.min(b.x_max),
        y_max: a.y_max.min(b.y_max),
    };
    (r.x_min <= r.x_max && r.y_min <= r.y_max).then_some(r)
}

/// Bit `j` is set if box `j` overlaps `r` (borders included), for up to 64 boxes.
#[inline(always)]
#[allow(clippy::needless_bitwise_bool)] // branch-free
pub fn rect_mask(x_min: &[f32], y_min: &[f32], x_max: &[f32], y_max: &[f32], r: Rect) -> u64 {
    debug_assert!(x_min.len() <= 64);
    let mut mask = 0;
    for (j, (((&x0, &y0), &x1), &y1)) in x_min.iter().zip(y_min).zip(x_max).zip(y_max).enumerate() {
        let overlaps = (x0 <= r.x_max) & (x1 >= r.x_min) & (y0 <= r.y_max) & (y1 >= r.y_min);
        mask |= u64::from(overlaps) << j;
    }
    mask
}

/// Cohen-Sutherland outcode of `p` relative to `r`: one bit per side of `r` that `p` lies beyond.
#[inline(always)]
fn outcode(p: Point, r: Rect) -> u8 {
    u8::from(p.0 < r.x_min)
        | (u8::from(p.0 > r.x_max) << 1)
        | (u8::from(p.1 < r.y_min) << 2)
        | (u8::from(p.1 > r.y_max) << 3)
}
