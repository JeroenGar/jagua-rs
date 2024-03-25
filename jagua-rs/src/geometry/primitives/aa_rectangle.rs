use std::cmp::Ordering;

use ordered_float::NotNan;

use crate::fsize;
use crate::geometry::geo_enums::{GeoPosition, GeoRelation};
use crate::geometry::geo_traits::{AlmostCollidesWith, CollidesWith, DistanceFrom, Shape};
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::util::fpa::FPA;

///Geometric primitive representing an axis-aligned rectangle
#[derive(Clone, Debug, PartialEq)]
pub struct AARectangle {
    pub x_min: fsize,
    pub y_min: fsize,
    pub x_max: fsize,
    pub y_max: fsize,
}

impl AARectangle {
    pub fn new(x_min: fsize, y_min: fsize, x_max: fsize, y_max: fsize) -> Self {
        debug_assert!(
            x_min < x_max && y_min < y_max,
            "invalid AARectangle, x_min: {}, x_max: {}, y_min: {}, y_max: {}",
            x_min,
            x_max,
            y_min,
            y_max
        );
        AARectangle {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn top_edge(&self) -> Edge {
        Edge::new(Point(self.x_max, self.y_max), Point(self.x_min, self.y_max))
    }

    pub fn bottom_edge(&self) -> Edge {
        Edge::new(Point(self.x_min, self.y_min), Point(self.x_max, self.y_min))
    }

    pub fn left_edge(&self) -> Edge {
        Edge::new(Point(self.x_min, self.y_max), Point(self.x_min, self.y_min))
    }

    pub fn right_edge(&self) -> Edge {
        Edge::new(Point(self.x_max, self.y_min), Point(self.x_max, self.y_max))
    }

    pub fn edges(&self) -> [Edge; 4] {
        [
            self.top_edge(),
            self.bottom_edge(),
            self.left_edge(),
            self.right_edge(),
        ]
    }

    pub fn corners(&self) -> [Point; 4] {
        [
            Point(self.x_min, self.y_max),
            Point(self.x_min, self.y_min),
            Point(self.x_max, self.y_min),
            Point(self.x_max, self.y_max),
        ]
    }

    /// Returns the relation between self and another AARectangle
    pub fn relation_to(&self, other: &AARectangle) -> GeoRelation {
        if self.collides_with(other) {
            if self.x_min <= other.x_min
                && self.y_min <= other.y_min
                && self.x_max >= other.x_max
                && self.y_max >= other.y_max
            {
                GeoRelation::Surrounding
            } else if self.x_min >= other.x_min
                && self.y_min >= other.y_min
                && self.x_max <= other.x_max
                && self.y_max <= other.y_max
            {
                GeoRelation::Enclosed
            } else {
                GeoRelation::Intersecting
            }
        } else {
            GeoRelation::Disjoint
        }
    }

    /// Returns the relation between self and another AARectangle, with a tolerance for floating point precision.
    /// Leaning towards `Surrounding` and `Enclosed` instead of `Intersecting` in edge cases.
    pub fn almost_relation_to(&self, other: &AARectangle) -> GeoRelation {
        if self.almost_collides_with(other) {
            if FPA::from(self.x_min) <= FPA::from(other.x_min)
                && FPA::from(self.y_min) <= FPA::from(other.y_min)
                && FPA::from(self.x_max) >= FPA::from(other.x_max)
                && FPA::from(self.y_max) >= FPA::from(other.y_max)
            {
                GeoRelation::Surrounding
            } else if FPA::from(self.x_min) >= FPA::from(other.x_min)
                && FPA::from(self.y_min) >= FPA::from(other.y_min)
                && FPA::from(self.x_max) <= FPA::from(other.x_max)
                && FPA::from(self.y_max) <= FPA::from(other.y_max)
            {
                GeoRelation::Enclosed
            } else {
                GeoRelation::Intersecting
            }
        } else {
            GeoRelation::Disjoint
        }
    }

    /// Returns the rectangle that is the result of inflating the smallest dimension of the rectangle to match the largest dimension
    pub fn inflate_to_square(&self) -> AARectangle {
        let width = self.x_max - self.x_min;
        let height = self.y_max - self.y_min;
        let mut dx = 0.0;
        let mut dy = 0.0;
        if height < width {
            dy = (width - height) / 2.0;
        } else if width < height {
            dx = (height - width) / 2.0;
        }
        AARectangle::new(
            self.x_min - dx,
            self.y_min - dy,
            self.x_max + dx,
            self.y_max + dy,
        )
    }

    pub fn scale(mut self, factor: fsize) -> Self {
        let dx = (self.x_max - self.x_min) * (factor - 1.0) / 2.0;
        let dy = (self.y_max - self.y_min) * (factor - 1.0) / 2.0;
        self.x_min -= dx;
        self.y_min -= dy;
        self.x_max += dx;
        self.y_max += dy;

        self
    }

    /// For all quadrants, contains indices of the two neighbors of the quadrant at that index
    pub const QUADRANT_NEIGHBOR_LAYOUT: [[usize; 2]; 4] = [[1, 2], [0, 3], [0, 3], [1, 2]];

    /// Returns the 4 quadrants of the rectangle, in the order NW, NE, SW, SE
    pub fn quadrants(&self) -> [Self; 4] {
        let Point(x_mid, y_mid) = self.centroid();
        let (x_min, y_min, x_max, y_max) = (
            self.x_min.into(),
            self.y_min.into(),
            self.x_max.into(),
            self.y_max.into(),
        );

        let rect_nw = AARectangle::new(x_min, y_mid, x_mid, y_max);
        let rect_ne = AARectangle::new(x_mid, y_mid, x_max, y_max);
        let rect_sw = AARectangle::new(x_min, y_min, x_mid, y_mid);
        let rect_se = AARectangle::new(x_mid, y_min, x_max, y_mid);

        [rect_nw, rect_ne, rect_sw, rect_se]
    }

    pub fn width(&self) -> fsize {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> fsize {
        self.y_max - self.y_min
    }

    pub fn from_intersection(a: &AARectangle, b: &AARectangle) -> Option<AARectangle> {
        let x_min = fsize::max(a.x_min, b.x_min);
        let y_min = fsize::max(a.y_min, b.y_min);
        let x_max = fsize::min(a.x_max, b.x_max);
        let y_max = fsize::min(a.y_max, b.y_max);
        if x_min < x_max && y_min < y_max {
            Some(AARectangle::new(x_min, y_min, x_max, y_max))
        } else {
            None
        }
    }

    pub fn from_union(a: &AARectangle, b: &AARectangle) -> AARectangle {
        let x_min = fsize::min(a.x_min, b.x_min);
        let y_min = fsize::min(a.y_min, b.y_min);
        let x_max = fsize::max(a.x_max, b.x_max);
        let y_max = fsize::max(a.y_max, b.y_max);
        AARectangle::new(x_min, y_min, x_max, y_max)
    }
}

impl Shape for AARectangle {
    fn centroid(&self) -> Point {
        Point(
            (self.x_min + self.x_max) / 2.0,
            (self.y_min + self.y_max) / 2.0,
        )
    }

    fn area(&self) -> fsize {
        (self.x_max - self.x_min) * (self.y_max - self.y_min)
    }

    fn bbox(&self) -> AARectangle {
        self.clone()
    }

    fn diameter(&self) -> fsize {
        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

impl CollidesWith<AARectangle> for AARectangle {
    fn collides_with(&self, other: &AARectangle) -> bool {
        fsize::max(self.x_min, other.x_min) <= fsize::min(self.x_max, other.x_max)
            && fsize::max(self.y_min, other.y_min) <= fsize::min(self.y_max, other.y_max)
    }
}

impl AlmostCollidesWith<AARectangle> for AARectangle {
    fn almost_collides_with(&self, other: &AARectangle) -> bool {
        FPA(fsize::max(self.x_min, other.x_min)) <= FPA(fsize::min(self.x_max, other.x_max))
            && FPA(fsize::max(self.y_min, other.y_min)) <= FPA(fsize::min(self.y_max, other.y_max))
    }
}

impl CollidesWith<Point> for AARectangle {
    fn collides_with(&self, point: &Point) -> bool {
        let (x, y) = (*point).into();
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

impl AlmostCollidesWith<Point> for AARectangle {
    fn almost_collides_with(&self, point: &Point) -> bool {
        let (x, y) = (*point).into();
        FPA(x) >= FPA(self.x_min)
            && FPA(x) <= FPA(self.x_max)
            && FPA(y) >= FPA(self.y_min)
            && FPA(y) <= FPA(self.y_max)
    }
}

impl CollidesWith<Edge> for AARectangle {
    #[inline(always)]
    fn collides_with(&self, edge: &Edge) -> bool {
        //inspired by: https://stackoverflow.com/questions/99353/how-to-test-if-a-line-segment-intersects-an-axis-aligned-rectange-in-2d

        let Point(x1, y1) = edge.start;
        let Point(x2, y2) = edge.end;

        //If either end point of the line is inside the rectangle
        if self.collides_with(&edge.start) || self.collides_with(&edge.end) {
            return true;
        }

        //If both end points of the line are entirely outside the range of the rectangle
        if x1 < self.x_min && x2 < self.x_min {
            return false; //edge entirely left of rectangle
        }
        if x1 > self.x_max && x2 > self.x_max {
            return false; //edge entirely right of rectangle
        }
        if y1 < self.y_min && y2 < self.y_min {
            return false; //edge entirely above rectangle
        }
        if y1 > self.y_max && y2 > self.y_max {
            return false; //edge entirely below rectangle
        }

        const POINT_EDGE_RELATION: fn(Point, &Edge) -> fsize = |p: Point, edge: &Edge| -> fsize {
            // if 0.0, the point is on the line
            // if > 0.0, the point is "above" of the line
            // if < 0.0, the point is "below" the line
            let (p_x, p_y) = p.into();
            let (s_x, s_y) = edge.start.into();
            let (e_x, e_y) = edge.end.into();
            (p_x - s_x) * (e_y - s_y) - (p_y - s_y) * (e_x - s_x)
        };

        //if all 4 corners of the rectangle are on the same side of the line, there is no intersection
        let mut ordering = None;
        for corner in self.corners() {
            match (
                ordering,
                POINT_EDGE_RELATION(corner, edge).partial_cmp(&0.0).unwrap(),
            ) {
                (Some(Ordering::Greater), Ordering::Greater) => (), //same side as previous corner,
                (Some(Ordering::Less), Ordering::Less) => (),       //same side as previous corner,
                (_, Ordering::Equal) => {
                    //corner is on the extended line, but not on the edge itself
                    ordering = None;
                    break;
                }
                (Some(_), _) => {
                    //not all corners are on the same side of the line
                    ordering = None;
                    break;
                }
                (None, rel) => ordering = Some(rel), //first corner, set the ordering
            }
        }
        if ordering.is_some() {
            //all points of the AARectangle reside on the same side of the edge, so there is no collision
            return false;
        }

        //The only possible that remains is that the edge collides with one of the edges of the AARectangle

        //If the line intersects with one of the edges of the rectangle
        edge.collides_with(&self.top_edge())
            || edge.collides_with(&self.bottom_edge())
            || edge.collides_with(&self.right_edge())
            || edge.collides_with(&self.left_edge())
    }
}

impl DistanceFrom<Point> for AARectangle {
    fn sq_distance(&self, point: &Point) -> fsize {
        let Point(x, y) = *point;
        let mut distance: fsize = 0.0;
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

    fn distance(&self, point: &Point) -> fsize {
        self.sq_distance(point).sqrt()
    }

    fn distance_from_border(&self, point: &Point) -> (GeoPosition, fsize) {
        let (position, sq_distance) = self.sq_distance_from_border(point);
        (position, sq_distance.sqrt())
    }

    fn sq_distance_from_border(&self, point: &Point) -> (GeoPosition, fsize) {
        match self.collides_with(point) {
            false => (GeoPosition::Exterior, self.sq_distance(point)),
            true => {
                let (x, y) = (NotNan::new(point.0).unwrap(), NotNan::new(point.1).unwrap());
                let distances = [
                    (x - self.x_min).abs(),
                    (x - self.x_max).abs(),
                    (y - self.y_min).abs(),
                    (y - self.y_max).abs(),
                ];
                let min = distances
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                (GeoPosition::Interior, *min)
            }
        }
    }
}
