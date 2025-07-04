use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{
    AlmostCollidesWith, CollidesWith, DistanceTo, SeparationDistance,
};
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::Point;
use crate::util::FPA;
use anyhow::Result;
use anyhow::ensure;
use ordered_float::OrderedFloat;

///Axis-aligned rectangle
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Rect {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

impl Rect {
    pub fn try_new(x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> Result<Self> {
        ensure!(
            x_min < x_max && y_min < y_max,
            "invalid rectangle, x_min: {x_min}, x_max: {x_max}, y_min: {y_min}, y_max: {y_max}"
        );
        Ok(Rect {
            x_min,
            y_min,
            x_max,
            y_max,
        })
    }

    pub fn from_diagonal_corners(c1: Point, c2: Point) -> Result<Self> {
        let x_min = f32::min(c1.x(), c2.x());
        let y_min = f32::min(c1.y(), c2.y());
        let x_max = f32::max(c1.x(), c2.x());
        let y_max = f32::max(c1.y(), c2.y());
        Rect::try_new(x_min, y_min, x_max, y_max)
    }

    /// Returns the geometric relation between `self` and another [`Rect`].
    /// Optimized for `GeoRelation::Disjoint`
    #[inline(always)]
    pub fn relation_to(&self, other: Rect) -> GeoRelation {
        if !self.collides_with(&other) {
            return GeoRelation::Disjoint;
        }
        if self.x_min <= other.x_min
            && self.y_min <= other.y_min
            && self.x_max >= other.x_max
            && self.y_max >= other.y_max
        {
            return GeoRelation::Surrounding;
        }
        if self.x_min >= other.x_min
            && self.y_min >= other.y_min
            && self.x_max <= other.x_max
            && self.y_max <= other.y_max
        {
            return GeoRelation::Enclosed;
        }
        GeoRelation::Intersecting
    }

    /// Returns the [`GeoRelation`] between `self` and another [`Rect`], with a tolerance for floating point precision.
    /// In edge cases, this method will lean towards `Surrounding` and `Enclosed` instead of `Intersecting`.
    #[inline(always)]
    pub fn almost_relation_to(&self, other: Rect) -> GeoRelation {
        if !self.almost_collides_with(&other) {
            return GeoRelation::Disjoint;
        }
        if FPA::from(self.x_min) <= FPA::from(other.x_min)
            && FPA::from(self.y_min) <= FPA::from(other.y_min)
            && FPA::from(self.x_max) >= FPA::from(other.x_max)
            && FPA::from(self.y_max) >= FPA::from(other.y_max)
        {
            return GeoRelation::Surrounding;
        }
        if FPA::from(self.x_min) >= FPA::from(other.x_min)
            && FPA::from(self.y_min) >= FPA::from(other.y_min)
            && FPA::from(self.x_max) <= FPA::from(other.x_max)
            && FPA::from(self.y_max) <= FPA::from(other.y_max)
        {
            return GeoRelation::Enclosed;
        }
        GeoRelation::Intersecting
    }

    /// Returns a new rectangle with the same centroid but inflated
    /// to be the minimum square that contains `self`.
    pub fn inflate_to_square(&self) -> Rect {
        let width = self.x_max - self.x_min;
        let height = self.y_max - self.y_min;
        let mut dx = 0.0;
        let mut dy = 0.0;
        if height < width {
            dy = (width - height) / 2.0;
        } else if width < height {
            dx = (height - width) / 2.0;
        }
        Rect {
            x_min: self.x_min - dx,
            y_min: self.y_min - dy,
            x_max: self.x_max + dx,
            y_max: self.y_max + dy,
        }
    }

    /// Returns a new rectangle with the same centroid but scaled by `factor`.
    pub fn scale(self, factor: f32) -> Self {
        let dx = (self.x_max - self.x_min) * (factor - 1.0) / 2.0;
        let dy = (self.y_max - self.y_min) * (factor - 1.0) / 2.0;
        self.resize_by(dx, dy)
            .expect("scaling should not lead to invalid rectangle")
    }

    /// Returns a new rectangle with the same centroid as `self` but expanded by `dx` in both x-directions and by `dy` in both y-directions.
    /// If the new rectangle is invalid (x_min >= x_max or y_min >= y_max), returns None.
    pub fn resize_by(mut self, dx: f32, dy: f32) -> Option<Self> {
        self.x_min -= dx;
        self.y_min -= dy;
        self.x_max += dx;
        self.y_max += dy;

        if self.x_min < self.x_max && self.y_min < self.y_max {
            Some(self)
        } else {
            //resizing would lead to invalid rectangle
            None
        }
    }

    /// For all quadrants, contains indices of the two neighbors of the quadrant at that index.
    pub const QUADRANT_NEIGHBOR_LAYOUT: [[usize; 2]; 4] = [[1, 3], [0, 2], [1, 3], [0, 2]];

    /// Returns the 4 quadrants of `self`.
    /// Ordered in the same way as quadrants in a cartesian plane:
    /// <https://en.wikipedia.org/wiki/Quadrant_(plane_geometry)>
    pub fn quadrants(&self) -> [Self; 4] {
        let mid = self.centroid();
        let corners = self.corners();

        let q1 = Rect::from_diagonal_corners(corners[0], mid).unwrap();
        let q2 = Rect::from_diagonal_corners(corners[1], mid).unwrap();
        let q3 = Rect::from_diagonal_corners(corners[2], mid).unwrap();
        let q4 = Rect::from_diagonal_corners(corners[3], mid).unwrap();

        [q1, q2, q3, q4]
    }

    /// Returns the four corners of `self`, in the same order as [Rect::quadrants].
    pub fn corners(&self) -> [Point; 4] {
        [
            Point(self.x_max, self.y_max),
            Point(self.x_min, self.y_max),
            Point(self.x_min, self.y_min),
            Point(self.x_max, self.y_min),
        ]
    }

    /// Returns the four edges that make up `self`, in the same order as [Rect::quadrants].
    pub fn edges(&self) -> [Edge; 4] {
        let c = self.corners();
        [
            Edge {
                start: c[0],
                end: c[1],
            },
            Edge {
                start: c[1],
                end: c[2],
            },
            Edge {
                start: c[2],
                end: c[3],
            },
            Edge {
                start: c[3],
                end: c[0],
            },
        ]
    }
    pub fn width(&self) -> f32 {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> f32 {
        self.y_max - self.y_min
    }

    /// Returns the largest rectangle that is contained in both `a` and `b`.
    pub fn intersection(a: Rect, b: Rect) -> Option<Rect> {
        let x_min = f32::max(a.x_min, b.x_min);
        let y_min = f32::max(a.y_min, b.y_min);
        let x_max = f32::min(a.x_max, b.x_max);
        let y_max = f32::min(a.y_max, b.y_max);
        if x_min < x_max && y_min < y_max {
            Some(Rect {
                x_min,
                y_min,
                x_max,
                y_max,
            })
        } else {
            None
        }
    }

    /// Returns the smallest rectangle that contains both `a` and `b`.
    pub fn bounding_rect(a: Rect, b: Rect) -> Rect {
        let x_min = f32::min(a.x_min, b.x_min);
        let y_min = f32::min(a.y_min, b.y_min);
        let x_max = f32::max(a.x_max, b.x_max);
        let y_max = f32::max(a.y_max, b.y_max);
        Rect {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn centroid(&self) -> Point {
        Point(
            (self.x_min + self.x_max) / 2.0,
            (self.y_min + self.y_max) / 2.0,
        )
    }

    pub fn area(&self) -> f32 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min)
    }

    pub fn diameter(&self) -> f32 {
        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

impl CollidesWith<Rect> for Rect {
    #[inline(always)]
    fn collides_with(&self, other: &Rect) -> bool {
        f32::max(self.x_min, other.x_min) <= f32::min(self.x_max, other.x_max)
            && f32::max(self.y_min, other.y_min) <= f32::min(self.y_max, other.y_max)
    }
}

impl AlmostCollidesWith<Rect> for Rect {
    #[inline(always)]
    fn almost_collides_with(&self, other: &Rect) -> bool {
        FPA(f32::max(self.x_min, other.x_min)) <= FPA(f32::min(self.x_max, other.x_max))
            && FPA(f32::max(self.y_min, other.y_min)) <= FPA(f32::min(self.y_max, other.y_max))
    }
}

impl CollidesWith<Point> for Rect {
    #[inline(always)]
    fn collides_with(&self, point: &Point) -> bool {
        let Point(x, y) = *point;
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

impl AlmostCollidesWith<Point> for Rect {
    #[inline(always)]
    fn almost_collides_with(&self, point: &Point) -> bool {
        let (x, y) = (*point).into();
        FPA(x) >= FPA(self.x_min)
            && FPA(x) <= FPA(self.x_max)
            && FPA(y) >= FPA(self.y_min)
            && FPA(y) <= FPA(self.y_max)
    }
}

impl CollidesWith<Edge> for Rect {
    #[inline(always)]
    #[cfg(not(feature = "simd"))]
    fn collides_with(&self, edge: &Edge) -> bool {
        //inspired by: https://stackoverflow.com/questions/99353/how-to-test-if-a-line-segment-intersects-an-axis-aligned-rectange-in-2d

        let e_x_min = edge.x_min();
        let e_x_max = edge.x_max();
        let e_y_min = edge.y_min();
        let e_y_max = edge.y_max();

        let x_no_overlap = e_x_min.max(self.x_min) > e_x_max.min(self.x_max);
        let y_no_overlap = e_y_min.max(self.y_min) > e_y_max.min(self.y_max);

        if x_no_overlap || y_no_overlap {
            return false; // Bounding boxes do not overlap
        }

        //If either end point of the line is inside the rectangle
        if self.collides_with(&edge.start) || self.collides_with(&edge.end) {
            return true;
        }

        //Determine which side of the edge each corner of the rectangle is on
        let corner_sides = self.corners().map(|corner| {
            let Point(p_x, p_y) = corner;
            let Point(s_x, s_y) = edge.start;
            let Point(e_x, e_y) = edge.end;
            // if 0.0, the corner is on the edge
            // if > 0.0, the corner is "above" of the edge
            // if < 0.0, the corner is "below" the edge
            (p_x - s_x) * (e_y - s_y) - (p_y - s_y) * (e_x - s_x)
        });

        //No collision if all corners are on the same side of the edge
        if corner_sides.iter().all(|v| *v > 0.0) || corner_sides.iter().all(|v| *v < 0.0) {
            return false;
        }

        //The only possible that remains is that the edge collides with one of the edges of the rectangle
        self.edges()
            .iter()
            .any(|rect_edge| edge.collides_with(rect_edge))
    }

    #[inline(always)]
    #[cfg(feature = "simd")]
    fn collides_with(&self, edge: &Edge) -> bool {
        {
            // SIMD: Check bounding box overlap
            if simd_bbox_no_overlap(self, edge) {
                return false;
            }

            // SIMD: Check if endpoints are inside rectangle
            if simd_endpoints_inside(self, edge) {
                return true;
            }

            // SIMD: Check if all corners are on same side
            if simd_all_corners_same_side(self, edge) {
                return false;
            }

            // SIMD: Check rectangle edges intersection
            simd_rect_edges_intersect(self, edge)
        }
    }
}

// SIMD implementations
#[cfg(feature = "simd")]
mod simd_impl {
    use super::*;

    pub fn simd_bbox_no_overlap(rect: &Rect, edge: &Edge) -> bool {
        use std::arch::x86_64::*;

        unsafe {
            let rect_x_min = _mm_set1_ps(rect.x_min);
            let rect_x_max = _mm_set1_ps(rect.x_max);
            let rect_y_min = _mm_set1_ps(rect.y_min);
            let rect_y_max = _mm_set1_ps(rect.y_max);

            let edge_x_min = _mm_set1_ps(edge.x_min());
            let edge_x_max = _mm_set1_ps(edge.x_max());
            let edge_y_min = _mm_set1_ps(edge.y_min());
            let edge_y_max = _mm_set1_ps(edge.y_max());

            // Check for NO overlap (same as scalar logic)
            let x_no_overlap = _mm_cmpgt_ps(
                _mm_max_ps(edge_x_min, rect_x_min),
                _mm_min_ps(edge_x_max, rect_x_max),
            );
            let y_no_overlap = _mm_cmpgt_ps(
                _mm_max_ps(edge_y_min, rect_y_min),
                _mm_min_ps(edge_y_max, rect_y_max),
            );

            // Return true if there's NO overlap (same as scalar)
            // Use OR instead of AND, and check if any bit is set
            _mm_movemask_ps(_mm_or_ps(x_no_overlap, y_no_overlap)) != 0
        }
    }

    pub fn simd_endpoints_inside(rect: &Rect, edge: &Edge) -> bool {
        use std::arch::x86_64::*;

        unsafe {
            let start_x = _mm_set1_ps(edge.start.0);
            let start_y = _mm_set1_ps(edge.start.1);
            let end_x = _mm_set1_ps(edge.end.0);
            let end_y = _mm_set1_ps(edge.end.1);
            let rect_x_min = _mm_set1_ps(rect.x_min);
            let rect_x_max = _mm_set1_ps(rect.x_max);
            let rect_y_min = _mm_set1_ps(rect.y_min);
            let rect_y_max = _mm_set1_ps(rect.y_max);

            // Check if start point is inside (x >= min && x <= max && y >= min && y <= max)
            let start_inside = _mm_and_ps(
                _mm_and_ps(
                    _mm_cmpge_ps(start_x, rect_x_min),
                    _mm_cmple_ps(start_x, rect_x_max),
                ),
                _mm_and_ps(
                    _mm_cmpge_ps(start_y, rect_y_min),
                    _mm_cmple_ps(start_y, rect_y_max),
                ),
            );

            let end_inside = _mm_and_ps(
                _mm_and_ps(
                    _mm_cmpge_ps(end_x, rect_x_min),
                    _mm_cmple_ps(end_x, rect_x_max),
                ),
                _mm_and_ps(
                    _mm_cmpge_ps(end_y, rect_y_min),
                    _mm_cmple_ps(end_y, rect_y_max),
                ),
            );

            // Return true if either point is inside (any bit set)
            _mm_movemask_ps(start_inside) != 0 || _mm_movemask_ps(end_inside) != 0
        }
    }

    pub fn simd_all_corners_same_side(rect: &Rect, edge: &Edge) -> bool {
        use std::arch::x86_64::*;

        unsafe {
            let corners = rect.corners();
            // Match WASM order: corners[0], corners[1], corners[2], corners[3]
            let corner_x = _mm_set_ps(corners[0].0, corners[1].0, corners[2].0, corners[3].0);
            let corner_y = _mm_set_ps(corners[0].1, corners[1].1, corners[2].1, corners[3].1);

            let start_x = _mm_set1_ps(edge.start.0);
            let start_y = _mm_set1_ps(edge.start.1);
            let end_x = _mm_set1_ps(edge.end.0);
            let end_y = _mm_set1_ps(edge.end.1);

            let dx = _mm_sub_ps(corner_x, start_x);
            let dy = _mm_sub_ps(corner_y, start_y);
            let edge_dx = _mm_sub_ps(end_x, start_x);
            let edge_dy = _mm_sub_ps(end_y, start_y);

            let term1 = _mm_mul_ps(dx, edge_dy);
            let term2 = _mm_mul_ps(dy, edge_dx);
            let corner_sides = _mm_sub_ps(term1, term2);

            let zero = _mm_set1_ps(0.0);
            let all_positive = _mm_cmpgt_ps(corner_sides, zero);
            let all_negative = _mm_cmplt_ps(corner_sides, zero);

            // Check if all corners are on the same side
            _mm_movemask_ps(all_positive) == 0b1111 || _mm_movemask_ps(all_negative) == 0b1111
        }
    }

    pub fn simd_rect_edges_intersect(rect: &Rect, edge: &Edge) -> bool {
        // For x86_64, fall back to scalar implementation for edge-edge intersection
        // as the SIMD implementation is complex and error-prone
        // TODO: Implement SIMD edge-edge intersection
        rect.edges()
            .iter()
            .any(|rect_edge| edge.collides_with(rect_edge))
    }
}

#[cfg(feature = "simd")]
use simd_impl::*;

impl DistanceTo<Point> for Rect {
    #[inline(always)]
    fn distance_to(&self, point: &Point) -> f32 {
        self.sq_distance_to(point).sqrt()
    }

    #[inline(always)]
    fn sq_distance_to(&self, point: &Point) -> f32 {
        let Point(x, y) = *point;
        let mut distance: f32 = 0.0;
        if x < self.x_min {
            distance += (x - self.x_min).powi(2);
        } else if x > self.x_max {
            distance += (x - self.x_max).powi(2);
        }
        if y < self.y_min {
            distance += (y - self.y_min).powi(2);
        } else if y > self.y_max {
            distance += (y - self.y_max).powi(2);
        }
        distance.abs()
    }
}

impl SeparationDistance<Point> for Rect {
    #[inline(always)]
    fn separation_distance(&self, point: &Point) -> (GeoPosition, f32) {
        let (position, sq_distance) = self.sq_separation_distance(point);
        (position, sq_distance.sqrt())
    }

    #[inline(always)]
    fn sq_separation_distance(&self, point: &Point) -> (GeoPosition, f32) {
        match self.collides_with(point) {
            false => (GeoPosition::Exterior, self.sq_distance_to(point)),
            true => {
                let Point(x, y) = *point;
                let min_distance = [
                    (x - self.x_min).abs(),
                    (x - self.x_max).abs(),
                    (y - self.y_min).abs(),
                    (y - self.y_max).abs(),
                ]
                .into_iter()
                .min_by_key(|&d| OrderedFloat(d))
                .unwrap();
                (GeoPosition::Interior, min_distance.powi(2))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_edge_collision_test() {
        // Rectangle at origin
        let rect = Rect::try_new(0.0, 0.0, 10.0, 10.0).unwrap();

        // Various edge configurations
        let edges = vec![
            // Edge completely inside
            Edge {
                start: Point(2.0, 2.0),
                end: Point(8.0, 8.0),
            },
            // Edge crossing rectangle
            Edge {
                start: Point(-5.0, 5.0),
                end: Point(15.0, 5.0),
            },
            // Edge touching corner
            Edge {
                start: Point(10.0, 10.0),
                end: Point(15.0, 15.0),
            },
            // Edge outside
            Edge {
                start: Point(-5.0, -5.0),
                end: Point(-2.0, -2.0),
            },
            // Edge parallel to sides
            Edge {
                start: Point(0.0, 15.0),
                end: Point(10.0, 15.0),
            },
        ];

        let results = edges
            .iter()
            .map(|edge| {
                let result = rect.collides_with(edge);
                println!("Edge: {:?}, Collision: {}", edge, result);
                result
            })
            .collect::<Vec<_>>();

        assert_eq!(results, vec![true, true, true, false, false]);
    }
}
