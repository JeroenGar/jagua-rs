use itertools::Itertools;
use ordered_float::{NotNan, OrderedFloat};

use crate::geometry::Transformation;
use crate::geometry::convex_hull::convex_hull_from_points;
use crate::geometry::fail_fast::{SPSurrogate, SPSurrogateConfig, compute_pole};
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, DistanceTo, SeparationDistance, Transformable, TransformableFrom,
};
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::GeometryError;
use crate::geometry::primitives::Point;
use crate::geometry::primitives::Rect;
use crate::util::FPA;
use crate::util::assertions;
use anyhow::{Result, bail};
use log::debug;

/// A Simple Polygon is a polygon that does not intersect itself and contains no holes.
/// It is a closed shape with a finite number of vertices and edges.
/// [read more](https://en.wikipedia.org/wiki/Simple_polygon)
#[derive(Clone, Debug)]
pub struct SPolygon {
    /// Vertices in counterclockwise order, including after reflection.
    pub(crate) vertices: Vec<Point>,
    /// Bounding box
    pub(crate) bbox: Rect,
    /// Area of its interior
    pub(crate) area: f32,
    /// Maximum distance between any two points in the polygon
    pub(crate) diameter: f32,
    /// [Pole of inaccessibility](https://en.wikipedia.org/wiki/Pole_of_inaccessibility) represented as a circle
    pub(crate) poi: Circle,
    /// Optional surrogate representation of the polygon (subset of the original)
    pub(crate) surrogate: Option<SPSurrogate>,
}

/// Result of [`SPolygon::remove_short_edges`].
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShortEdgeRemoval {
    /// Minimum edge length that was enforced
    pub min_length: f32,
    /// Removed vertices, in the order of the input points
    pub removed: Vec<RemovedVertex>,
}

/// A vertex removed by [`SPolygon::remove_short_edges`]. Indices refer to its input points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RemovedVertex {
    pub idx: usize,
    pub point: Point,
    /// Index of the kept vertex it lay too close to
    pub kept_idx: usize,
    pub kept_point: Point,
    pub distance: f32,
}

impl std::fmt::Display for ShortEdgeRemoval {
    /// Describes the removed vertices, listing at most [`ShortEdgeRemoval::MAX_LISTED`] of them.
    /// Coordinates are written like in the JSON input (`[x, y]`), so they can be searched for.
    /// They match the input text whenever it is exactly representable as `f32`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl ShortEdgeRemoval {
    /// Maximum number of removed vertices listed when displayed
    const MAX_LISTED: usize = 10;
}

impl SPolygon {
    /// Minimum length of an edge, as a fraction of the polygon's diameter.
    ///
    /// Shorter edges are removed on construction, see [`SPolygon::new`].
    /// The margin is large compared to the `f32` rounding error of the coordinates of a centered polygon
    /// (about `6e-8` times the diameter), so transformations cannot collapse an edge into a single point (issue #133).
    pub const MIN_EDGE_LENGTH_RATIO: f32 = 1e-5;

    /// Create a new simple polygon from a set of points, expensive operations are performed here! Use [`Self::clone()`] or [`Self::transform()`] to avoid recomputation.
    ///
    /// Every vertex that lies closer than [`SPolygon::MIN_EDGE_LENGTH_RATIO`] times the diameter
    /// to the previous kept vertex is removed, so no edge is shorter than that.
    /// Returns an error if the remaining vertices do not form a valid simple polygon.
    pub fn new(mut points: Vec<Point>) -> Result<Self> {
        if points.len() < 3 {
            bail!(GeometryError::TooFewPoints(points.len()));
        }
        let removal = SPolygon::remove_short_edges(&mut points);
        if !removal.removed.is_empty() {
            debug!("[SP] {removal}");
        }
        if points.len() < 3 {
            bail!(GeometryError::TooFewPoints(points.len()));
        }
        if points.iter().unique().count() != points.len() {
            let [i, j] = (0..points.len())
                .array_combinations()
                .find(|&[i, j]| points[i] == points[j])
                .expect("a duplicate point exists");
            bail!(GeometryError::DuplicatePoints(i, j));
        }
        if let Some((e1_idx, e2_idx)) = SPolygon::find_self_intersection(&points) {
            bail!(GeometryError::IntersectingEdges(e1_idx, e2_idx));
        }

        let area = match SPolygon::calculate_area(&points) {
            0.0 => bail!(GeometryError::NoArea),
            area if area < 0.0 => {
                //edges should always be ordered counterclockwise (positive area)
                points.reverse();
                -area
            }
            area => area,
        };

        let diameter = SPolygon::calculate_diameter(points.clone());
        let bbox = SPolygon::generate_bounding_box(&points);
        let poi = SPolygon::calculate_poi(&points, diameter)?;

        Ok(SPolygon {
            vertices: points,
            bbox,
            area,
            diameter,
            poi,
            surrogate: None,
        })
    }

    pub fn generate_surrogate(&mut self, config: SPSurrogateConfig) -> Result<()> {
        //regenerate the surrogate if it is not present or if the config has changed
        match &self.surrogate {
            Some(surrogate) if surrogate.config == config => {}
            _ => self.surrogate = Some(SPSurrogate::new(self, config)?),
        }
        Ok(())
    }

    #[must_use]
    pub fn vertex(&self, i: usize) -> Point {
        self.vertices[i]
    }

    #[must_use]
    pub fn edge(&self, i: usize) -> Edge {
        assert!(i < self.n_vertices(), "index out of bounds");
        let j = if i == self.n_vertices() - 1 { 0 } else { i + 1 };
        Edge {
            start: self.vertices[i],
            end: self.vertices[j],
        }
    }

    pub fn edge_iter(&self) -> impl Iterator<Item = Edge> + '_ {
        (0..self.n_vertices()).map(move |i| self.edge(i))
    }

    #[must_use]
    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    #[must_use]
    pub fn surrogate(&self) -> &SPSurrogate {
        self.surrogate.as_ref().expect("surrogate not generated")
    }

    /// Removes every vertex that lies closer than [`SPolygon::MIN_EDGE_LENGTH_RATIO`] times the diameter
    /// of `points` to the previous kept vertex, including across the edge that closes the polygon.
    /// Comparing against the previous *kept* vertex thins out a dense run of short edges
    /// (e.g. a finely sampled arc) instead of removing it entirely.
    pub(crate) fn remove_short_edges(points: &mut Vec<Point>) -> ShortEdgeRemoval {
        let min_length =
            SPolygon::MIN_EDGE_LENGTH_RATIO * SPolygon::calculate_diameter(points.clone());
        let min_sq_length = min_length * min_length;

        let mut removed = vec![];
        let mut kept: Vec<(usize, Point)> = Vec::with_capacity(points.len());
        for (idx, &point) in points.iter().enumerate() {
            match kept.last() {
                Some(&(kept_idx, kept_point))
                    if kept_point.sq_distance_to(&point) < min_sq_length =>
                {
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

    #[must_use]
    fn calculate_diameter(points: Vec<Point>) -> f32 {
        //The two points furthest apart must be part of the convex hull
        let ch = convex_hull_from_points(points);

        //go through all pairs of points and find the pair with the largest distance
        let sq_diam = ch
            .iter()
            .array_combinations::<2>()
            .map(|[p1, p2]| p1.sq_distance_to(p2))
            .max_by_key(|sq_d| NotNan::new(*sq_d).unwrap())
            .unwrap_or(0.0); // fewer than two distinct points

        sq_diam.sqrt()
    }

    #[must_use]
    pub fn generate_bounding_box(points: &[Point]) -> Rect {
        let (mut x_min, mut y_min) = (f32::MAX, f32::MAX);
        let (mut x_max, mut y_max) = (f32::MIN, f32::MIN);

        for point in points {
            x_min = x_min.min(point.0);
            y_min = y_min.min(point.1);
            x_max = x_max.max(point.0);
            y_max = y_max.max(point.1);
        }
        Rect::try_new(x_min, y_min, x_max, y_max).unwrap()
    }

    //https://en.wikipedia.org/wiki/Shoelace_formula
    //counterclockwise = positive area, clockwise = negative area
    #[must_use]
    pub fn calculate_area(points: &[Point]) -> f32 {
        let mut sigma: f32 = 0.0;
        for i in 0..points.len() {
            //next point
            let j = (i + 1) % points.len();

            let (x_i, y_i) = points[i].into();
            let (x_j, y_j) = points[j].into();

            sigma += (y_i + y_j) * (x_i - x_j);
        }

        0.5 * sigma
    }

    fn calculate_poi(points: &[Point], diameter: f32) -> Result<Circle> {
        //need to make a dummy simple polygon, because the pole generation algorithm
        //relies on many of the methods provided by the simple polygon struct
        let dummy_sp = {
            let bbox = SPolygon::generate_bounding_box(points);
            let area = SPolygon::calculate_area(points);
            let dummy_poi = Circle::try_new(Point(f32::MAX, f32::MAX), f32::MAX).unwrap();

            SPolygon {
                vertices: points.to_vec(),
                bbox,
                area,
                diameter,
                poi: dummy_poi,
                surrogate: None,
            }
        };

        compute_pole(&dummy_sp, &[])
    }

    #[must_use]
    pub fn centroid(&self) -> Point {
        //based on: https://en.wikipedia.org/wiki/Centroid#Of_a_polygon

        let area = self.area;
        let mut c_x = 0.0;
        let mut c_y = 0.0;

        for i in 0..self.n_vertices() {
            let j = if i == self.n_vertices() - 1 { 0 } else { i + 1 };
            let Point(x_i, y_i) = self.vertex(i);
            let Point(x_j, y_j) = self.vertex(j);
            c_x += (x_i + x_j) * (x_i * y_j - x_j * y_i);
            c_y += (y_i + y_j) * (x_i * y_j - x_j * y_i);
        }

        c_x /= 6.0 * area;
        c_y /= 6.0 * area;

        (c_x, c_y).into()
    }

    fn find_self_intersection(points: &[Point]) -> Option<(usize, usize)> {
        let edge = |i| Edge {
            start: points[i],
            end: points[(i + 1) % points.len()],
        };
        let are_neighboring_edges = |i, j| i + 1 == j || (i == 0 && j == points.len() - 1);

        (0..points.len()).combinations(2).find_map(|pair| {
            let (i, j) = (pair[0], pair[1]);
            (!are_neighboring_edges(i, j) && edge(i).collides_with(&edge(j))).then_some((i, j))
        })
    }

    /// Vertices in counterclockwise order, including after reflection.
    #[must_use]
    pub fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    /// Bounding box
    #[must_use]
    pub fn bbox(&self) -> Rect {
        self.bbox
    }

    /// Area of its interior
    #[must_use]
    pub fn area(&self) -> f32 {
        self.area
    }

    /// Maximum distance between any two points in the polygon
    #[must_use]
    pub fn diameter(&self) -> f32 {
        self.diameter
    }

    /// [Pole of inaccessibility](https://en.wikipedia.org/wiki/Pole_of_inaccessibility) represented as a circle
    #[must_use]
    pub fn poi(&self) -> Circle {
        self.poi
    }
}

impl Transformable for SPolygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        let reflected = t.is_reflected();
        //destructuring pattern to ensure that the code is updated when the struct changes
        let SPolygon {
            vertices: points,
            bbox,
            area: _,
            diameter: _,
            poi,
            surrogate,
        } = self;

        //transform all points of the simple poly
        for p in points.iter_mut() {
            p.transform(t);
        }
        if reflected {
            points.reverse();
        }

        poi.transform(t);

        //transform the surrogate
        if let Some(surrogate) = surrogate.as_mut() {
            surrogate.transform(t);
            if reflected {
                surrogate.reverse_vertex_order(points.len());
            }
        }

        //regenerate bounding box
        *bbox = SPolygon::generate_bounding_box(points);

        self
    }
}

impl TransformableFrom for SPolygon {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        let reflected = t.is_reflected();
        assert_eq!(self.vertices.len(), reference.vertices.len());
        //destructuring pattern to ensure that the code is updated when the struct changes
        let SPolygon {
            vertices: points,
            bbox,
            area: _,
            diameter: _,
            poi,
            surrogate,
        } = self;

        if reflected {
            for (p, ref_p) in points.iter_mut().zip(reference.vertices.iter().rev()) {
                p.transform_from(ref_p, t);
            }
        } else {
            for (p, ref_p) in points.iter_mut().zip(&reference.vertices) {
                p.transform_from(ref_p, t);
            }
        }

        poi.transform_from(&reference.poi, t);

        //transform the surrogate
        if let Some(surrogate) = surrogate.as_mut() {
            let ref_surrogate = reference.surrogate();
            surrogate.transform_from(ref_surrogate, t);
            // The destination was cloned from `reference`, so its hull indices only
            // need remapping when the vertex storage orientation changes.
            if surrogate.hull_reversed() != (ref_surrogate.hull_reversed() ^ reflected) {
                surrogate.reverse_vertex_order(points.len());
            }
            debug_assert!(assertions::hull_indices_match_reference(
                surrogate,
                ref_surrogate,
                reflected,
                points.len()
            ));
        }
        //regenerate bounding box
        *bbox = SPolygon::generate_bounding_box(points);

        self
    }
}

impl CollidesWith<Point> for SPolygon {
    fn collides_with(&self, point: &Point) -> bool {
        //based on the ray casting algorithm: https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        if self.bbox.collides_with(point) {
            //horizontal ray shot to the right.
            //Starting from the point to another point that is certainly outside the shape
            let point_outside = Point(self.bbox.x_max + self.bbox.width(), point.1);
            let ray = Edge {
                start: *point,
                end: point_outside,
            };

            let n_intersections = self
                .edge_iter()
                .filter(|edge| ray_crosses(edge, point, &ray))
                .count();
            n_intersections % 2 == 1
        } else {
            false
        }
    }
}

/// Whether `edge` counts as a crossing of `ray`, shot horizontally to the right from `point`,
/// in the ray casting test of [`SPolygon::collides_with`].
#[inline(always)]
pub(crate) fn ray_crosses(edge: &Edge, point: &Point, ray: &Edge) -> bool {
    //Check if the ray does not go through (or almost through) a vertex
    //This can result in funky behaviour, which could incorrect results
    //Therefore we handle this case
    let (s_x, s_y) = (FPA(edge.start.0), FPA(edge.start.1));
    let (e_x, e_y) = (FPA(edge.end.0), FPA(edge.end.1));
    let (p_x, p_y) = (FPA(point.0), FPA(point.1));

    if (s_y == p_y && s_x > p_x) || (e_y == p_y && e_x > p_x) {
        //in this case, the ray passes through (or dangerously close to) a vertex
        //We handle this case by only counting an intersection if the edge is below the ray
        s_y < p_y || e_y < p_y
    } else {
        ray.collides_with(edge)
    }
}

impl DistanceTo<Point> for SPolygon {
    fn distance_to(&self, point: &Point) -> f32 {
        self.sq_distance_to(point).sqrt()
    }
    fn sq_distance_to(&self, point: &Point) -> f32 {
        if self.collides_with(point) {
            0.0
        } else {
            self.edge_iter()
                .map(|edge| edge.sq_distance_to(point))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }
    }
}

impl SeparationDistance<Point> for SPolygon {
    fn separation_distance(&self, point: &Point) -> (GeoPosition, f32) {
        let (position, sq_distance) = self.sq_separation_distance(point);
        (position, sq_distance.sqrt())
    }

    fn sq_separation_distance(&self, point: &Point) -> (GeoPosition, f32) {
        let distance_to_closest_edge = self
            .edge_iter()
            .map(|edge| edge.sq_distance_to(point))
            .min_by_key(|sq_d| OrderedFloat(*sq_d))
            .unwrap();

        if self.collides_with(point) {
            (GeoPosition::Interior, distance_to_closest_edge)
        } else {
            (GeoPosition::Exterior, distance_to_closest_edge)
        }
    }
}

impl TryFrom<&Rect> for SPolygon {
    type Error = anyhow::Error;

    /// Fails if the rectangle is so thin that [`SPolygon::new`] removes its short sides,
    /// see [`SPolygon::MIN_EDGE_LENGTH_RATIO`].
    fn try_from(r: &Rect) -> Result<Self> {
        SPolygon::new(vec![
            (r.x_min, r.y_min).into(),
            (r.x_max, r.y_min).into(),
            (r.x_max, r.y_max).into(),
            (r.x_min, r.y_max).into(),
        ])
    }
}

impl TryFrom<Rect> for SPolygon {
    type Error = anyhow::Error;

    /// See [`SPolygon::try_from(&Rect)`](#impl-TryFrom%3C%26Rect%3E-for-SPolygon).
    fn try_from(r: Rect) -> Result<Self> {
        SPolygon::try_from(&r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn points(coords: &[(f32, f32)]) -> Vec<Point> {
        coords.iter().map(|&(x, y)| Point(x, y)).collect()
    }

    fn assert_min_edge_length(sp: &SPolygon) {
        let min_edge_length = SPolygon::MIN_EDGE_LENGTH_RATIO * sp.diameter;
        assert!(sp.edge_iter().all(|e| e.length() >= min_edge_length));
    }

    #[test]
    fn new_removes_short_edges() {
        let square = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];

        // exact consecutive duplicate
        let sp = SPolygon::new(points(&[
            square[0], square[1], square[1], square[2], square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 4);

        // vertex closer than the minimum edge length to its predecessor
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            (10.0, 1e-5),
            square[2],
            square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 4);

        // last vertex too close to the first one (closing edge)
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            square[2],
            square[3],
            (0.0, 1e-5),
        ]));
        let sp = sp.unwrap();
        assert_eq!(sp.n_vertices(), 4);
        assert_min_edge_length(&sp);

        // a vertex at a safe distance is kept
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            (10.0, 1e-3),
            square[2],
            square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 5);
    }

    #[test]
    fn dense_run_of_short_edges_is_thinned_not_removed() {
        // A 10x10 square whose bottom edge starts with a run sampled every 1e-5,
        // far below the minimum edge length (~1.4e-4). Comparing against the previous kept vertex
        // should keep a vertex roughly every 1.4e-4 instead of removing the whole run.
        let mut pts = (0..=10_000u16)
            .map(|i| Point(f32::from(i) * 1e-5, 0.0))
            .collect_vec();
        pts.extend(points(&[(10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]));

        let n_removed = SPolygon::remove_short_edges(&mut pts).removed.len();

        let min_edge_length =
            SPolygon::MIN_EDGE_LENGTH_RATIO * SPolygon::calculate_diameter(pts.clone());
        assert!(n_removed > 9_000);
        assert!(
            pts.len() > 500,
            "the sampled edge should be thinned, not removed"
        );
        assert!(
            (0..pts.len())
                .all(|i| pts[i].distance_to(&pts[(i + 1) % pts.len()]) >= min_edge_length)
        );
    }

    #[test]
    fn removed_vertices_are_reported() {
        let mut pts = points(&[
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 1e-5),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 1e-5),
        ]);
        let removal = SPolygon::remove_short_edges(&mut pts);

        let reported = removal
            .removed
            .iter()
            .map(|r| (r.idx, r.kept_idx))
            .collect_vec();
        // vertex 2 is too close to vertex 1, vertex 5 too close to vertex 0 across the closing edge
        assert_eq!(reported, [(2, 1), (5, 0)]);
        assert_eq!(pts.len(), 4);
    }

    #[test]
    fn rectangle_conversion() {
        let rect = Rect::try_new(0.0, 0.0, 10.0, 5.0).unwrap();
        assert_eq!(SPolygon::try_from(rect).unwrap().n_vertices(), 4);

        // the short sides are below the minimum edge length, so they are removed
        let sliver = Rect::try_new(0.0, 0.0, 10.0, 1e-6).unwrap();
        assert!(SPolygon::try_from(&sliver).is_err());
    }

    #[test]
    fn new_fails_when_too_few_vertices_remain() {
        let sliver = points(&[(0.0, 0.0), (10.0, 0.0), (10.0, 1e-6), (0.0, 1e-6)]);
        assert!(SPolygon::new(sliver).is_err());
    }
}
