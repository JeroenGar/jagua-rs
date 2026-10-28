// The query loops rely on being inlined into their callers
#![allow(clippy::inline_always)]

use crate::collision_detection::hazard_index::rect_mask;
use crate::geometry::geo_traits::{CollidesWith, DistanceTo};
use crate::geometry::primitives::{Circle, Edge, Point, Rect, SPolygon, ray_crosses};
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
    let nv = vertices.len();
    // Edge `i` runs from vertex `i - 1` (the last one for `i = 0`) to vertex `i`. It can only
    // overlap `r` if its end points are not beyond the same side, one bit per vertex and side.
    let mut carry = beyond_mask(&vertices[nv - 1..], r);
    for start in (0..nv).step_by(64) {
        let end = (start + 64).min(nv);
        let beyond = beyond_mask(&vertices[start..end], r);
        let mut mask = u64::MAX >> (64 - (end - start));
        for (b, c) in beyond.iter().zip(&mut carry) {
            mask &= !(b & ((b << 1) | *c));
            *c = b >> (end - start - 1);
        }
        while mask != 0 {
            let i = start + mask.trailing_zeros() as usize;
            mask &= mask - 1;
            buf[n] = Edge {
                start: vertices[if i == 0 { nv - 1 } else { i - 1 }],
                end: vertices[i],
            };
            n += 1;
            if n == buf.len() {
                if edges.any_edge_overlapping(r, |e| buf.iter().any(|b| b.collides_with(e))) {
                    return true;
                }
                n = 0;
            }
        }
    }
    n > 0 && edges.any_edge_overlapping(r, |e| buf[..n].iter().any(|b| b.collides_with(e)))
}

/// Whether `poly` contains `p`, decided exactly as [`SPolygon::collides_with`] does, with
/// `edges` the edges of `poly`. The ray cast only visits edges whose bounding box reaches the
/// ray, widened in y by the tolerance of the approximate vertex test, since no other edge can
/// count as a crossing.
#[inline(always)]
pub fn poly_contains(poly: &SPolygon, edges: &impl EdgeSet, p: Point) -> bool {
    if !poly.bbox.collides_with(&p) {
        return false;
    }
    let ray = Edge {
        start: p,
        end: Point(poly.bbox.x_max + poly.bbox.width(), p.1),
    };
    // At least float_cmp's default margin: f32::EPSILON absolute or 4 ulps relative
    let margin = f32::EPSILON + 8.0 * f32::EPSILON * p.1.abs();
    let r = Rect {
        x_min: p.0,
        y_min: p.1 - margin,
        x_max: f32::INFINITY,
        y_max: p.1 + margin,
    };
    let mut crossings = 0;
    edges.any_edge_overlapping(r, |e| {
        crossings += usize::from(ray_crosses(e, &p, &ray));
        false
    });
    crossings % 2 == 1
}

/// Whether `c` touches an edge of `edges`.
#[inline(always)]
pub fn circle_touches_edges(c: &Circle, edges: &impl EdgeSet) -> bool {
    let sq_radius = c.radius.powi(2);
    edges.any_edge_overlapping(c.bbox(), |e| e.sq_distance_to(&c.center) <= sq_radius)
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

/// For each side of `r`, bit `j` is set if point `j` lies beyond it, for up to 64 points.
/// The sides are left, right, bottom and top.
#[cfg(jagua_simd)]
#[inline(always)]
fn beyond_mask(points: &[Point], r: Rect) -> [u64; 4] {
    use std::simd::prelude::*;
    type F = Simd<f32, 8>;
    let len = points.len();
    debug_assert!(len <= 64);
    if len < 8 {
        return beyond_mask_scalar(points, r);
    }
    let [x0, y0, x1, y1] = [r.x_min, r.y_min, r.x_max, r.y_max].map(F::splat);
    let block = |j: usize| {
        let x = F::from_array(std::array::from_fn(|k| points[j + k].0));
        let y = F::from_array(std::array::from_fn(|k| points[j + k].1));
        [x.simd_lt(x0), x.simd_gt(x1), y.simd_lt(y0), y.simd_gt(y1)].map(|m| m.to_bitmask() << j)
    };
    let mut mask = [0; 4];
    let mut or = |b: [u64; 4]| mask.iter_mut().zip(b).for_each(|(m, b)| *m |= b);
    for j in (0..len - len % 8).step_by(8) {
        or(block(j));
    }
    if !len.is_multiple_of(8) {
        // Partial last block: retest the last 8 points, which sets the same bits twice for some
        or(block(len - 8));
    }
    mask
}

/// For each side of `r`, bit `j` is set if point `j` lies beyond it, for up to 64 points.
/// The sides are left, right, bottom and top.
#[cfg(not(jagua_simd))]
#[inline(always)]
fn beyond_mask(points: &[Point], r: Rect) -> [u64; 4] {
    beyond_mask_scalar(points, r)
}

#[inline(always)]
fn beyond_mask_scalar(points: &[Point], r: Rect) -> [u64; 4] {
    debug_assert!(points.len() <= 64);
    let mut mask = [0; 4];
    for (j, &Point(x, y)) in points.iter().enumerate() {
        mask[0] |= u64::from(x < r.x_min) << j;
        mask[1] |= u64::from(x > r.x_max) << j;
        mask[2] |= u64::from(y < r.y_min) << j;
        mask[3] |= u64::from(y > r.y_max) << j;
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Xorshift values in `0..16`
    fn grid_values(mut seed: u32) -> impl FnMut() -> f32 {
        move || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            #[allow(clippy::cast_precision_loss)] // values below 16
            let v = (seed % 16) as f32;
            v
        }
    }

    #[test]
    fn beyond_mask_matches_scalar_for_every_length() {
        // Points on a grid, so they lie on the sides of `r` often
        let mut next = grid_values(0x2545_f491);
        let r = Rect {
            x_min: 4.0,
            y_min: 5.0,
            x_max: 9.0,
            y_max: 11.0,
        };
        for n in 0..=64 {
            let points = (0..n).map(|_| Point(next(), next())).collect::<Vec<_>>();
            assert_eq!(
                beyond_mask(&points, r),
                beyond_mask_scalar(&points, r),
                "n = {n}"
            );
        }
    }

    struct AllEdges(Vec<Edge>);

    impl EdgeSet for AllEdges {
        fn any_edge_overlapping(&self, r: Rect, mut f: impl FnMut(&Edge) -> bool) -> bool {
            self.0.iter().any(|e| e.bbox().collides_with(&r) && f(e))
        }
    }

    #[test]
    #[allow(clippy::cast_precision_loss)] // small counts
    fn edges_intersect_matches_brute_force() {
        let mut next = grid_values(0x9e37_79b9);
        for n in [3, 7, 8, 9, 63, 64, 65, 100, 128, 129, 150] {
            // A star-shaped polygon around (8, 8), with more than 64 vertices for some
            let vertices = (0..n)
                .map(|k| {
                    let angle = std::f32::consts::TAU * k as f32 / n as f32;
                    let radius = 2.0 + next() / 3.0;
                    Point(8.0 + radius * angle.cos(), 8.0 + radius * angle.sin())
                })
                .collect();
            let poly = SPolygon::new(vertices).unwrap();
            for _ in 0..50 {
                let edges = (0..10)
                    .map(|_| Edge {
                        start: Point(next(), next()),
                        end: Point(next(), next()),
                    })
                    .filter(|e| e.start != e.end)
                    .collect::<Vec<_>>();
                let (x, y) = ([next(), next()], [next(), next()]);
                let r = Rect {
                    x_min: x[0].min(x[1]),
                    y_min: y[0].min(y[1]),
                    x_max: x[0].max(x[1]),
                    y_max: y[0].max(y[1]),
                };
                let in_r = |e: &Edge| e.bbox().collides_with(&r);
                let expected = poly
                    .edge_iter()
                    .filter(in_r)
                    .any(|a| edges.iter().filter(|e| in_r(e)).any(|b| a.collides_with(b)));
                assert_eq!(
                    edges_intersect(&poly, r, &AllEdges(edges)),
                    expected,
                    "n = {n}"
                );
            }
        }
    }
}
