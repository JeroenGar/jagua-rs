use std::fmt;

use itertools::Itertools;

use crate::geometry::geo_traits::DistanceTo;
use crate::geometry::primitives::{Point, SPolygon};

/// Removes every vertex that lies closer than [`SPolygon::MIN_EDGE_LENGTH_RATIO`] times the diameter
/// of `points` to the previous kept vertex, including across the edge that closes the polygon.
/// Comparing against the previous *kept* vertex thins out a dense run of short edges
/// (e.g. a finely sampled arc) instead of removing it entirely.
pub(crate) fn remove_short_edges(points: &mut Vec<Point>) -> ShortEdgeRemoval {
    let min_length = min_edge_length(points);
    let min_sq_length = min_length * min_length;

    let mut removed = vec![];
    let mut kept: Vec<(usize, Point)> = Vec::with_capacity(points.len());
    for (idx, &point) in points.iter().enumerate() {
        match kept.last() {
            Some(&(kept_idx, kept_point)) if kept_point.sq_distance_to(&point) < min_sq_length => {
                removed.push(RemovedVertex {
                    idx,
                    point,
                    kept_idx,
                    kept_point,
                    distance: kept_point.distance_to(&point),
                });
            }
            _ => kept.push((idx, point)),
        }
    }
    // the closing edge runs from the last kept vertex back to the first one
    while kept.len() > 1 && kept[kept.len() - 1].1.sq_distance_to(&kept[0].1) < min_sq_length {
        let (idx, point) = kept.pop().expect("kept is not empty");
        removed.push(RemovedVertex {
            idx,
            point,
            kept_idx: kept[0].0,
            kept_point: kept[0].1,
            distance: kept[0].1.distance_to(&point),
        });
    }
    removed.sort_by_key(|r| r.idx);

    *points = kept.into_iter().map(|(_, p)| p).collect();
    ShortEdgeRemoval {
        min_length,
        removed,
    }
}

/// Whether [`remove_short_edges`] would remove any vertex of `points`.
pub(crate) fn has_short_edges(points: &[Point]) -> bool {
    let min_length = min_edge_length(points);
    let min_sq_length = min_length * min_length;
    points
        .iter()
        .circular_tuple_windows()
        .any(|(a, b)| a.sq_distance_to(b) < min_sq_length)
}

fn min_edge_length(points: &[Point]) -> f32 {
    SPolygon::MIN_EDGE_LENGTH_RATIO * SPolygon::calculate_diameter(points.to_vec())
}

/// Result of [`remove_short_edges`].
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShortEdgeRemoval {
    /// Minimum edge length that was enforced
    pub min_length: f32,
    /// Removed vertices, in the order of the input points
    pub removed: Vec<RemovedVertex>,
}

impl ShortEdgeRemoval {
    /// Maximum number of removed vertices listed when displayed
    const MAX_LISTED: usize = 10;
}

/// A vertex removed by [`remove_short_edges`]. Indices refer to its input points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RemovedVertex {
    pub idx: usize,
    pub point: Point,
    /// Index of the kept vertex it lay too close to
    pub kept_idx: usize,
    pub kept_point: Point,
    pub distance: f32,
}

impl fmt::Display for ShortEdgeRemoval {
    /// Describes the removed vertices, listing at most [`ShortEdgeRemoval::MAX_LISTED`] of them.
    /// Coordinates are written like in the JSON input (`[x, y]`), so they can be searched for.
    /// They match the input text whenever it is exactly representable as `f32`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.removed.len();
        write!(
            f,
            "removed {n} {} closer than {:?} ({:?} times the diameter) to the previous kept vertex: ",
            if n == 1 { "vertex" } else { "vertices" },
            self.min_length,
            SPolygon::MIN_EDGE_LENGTH_RATIO
        )?;
        for (i, r) in self.removed.iter().take(Self::MAX_LISTED).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "vertex {} [{:?}, {:?}] is {:?} from vertex {} [{:?}, {:?}]",
                r.idx, r.point.0, r.point.1, r.distance, r.kept_idx, r.kept_point.0, r.kept_point.1
            )?;
        }
        if n > Self::MAX_LISTED {
            write!(f, " and {} more", n - Self::MAX_LISTED)?;
        }
        Ok(())
    }
}
