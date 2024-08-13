use std::cmp::Ordering;

use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, DistanceFrom, Shape, Transformable, TransformableFrom,
};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::geometry::transformation::Transformation;
use crate::{fsize, PI};

/// Geometric primitive representing a circle
#[derive(Clone, Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: fsize,
}

impl Circle {
    pub fn new(center: Point, radius: fsize) -> Self {
        debug_assert!(
            radius.is_finite() && radius >= 0.0,
            "invalid circle radius: {}",
            radius
        );
        debug_assert!(
            center.0.is_finite() && center.1.is_finite(),
            "invalid circle center: {:?}",
            center
        );

        Self { center, radius }
    }

    /// Returns the smallest possible circle that fully contains all ```circles```
    pub fn bounding_circle<'a>(mut circles: impl IntoIterator<Item = &'a Circle>) -> Circle {
        let mut circles = circles.into_iter();
        let mut bounding_circle = circles.next().expect("no circles provided").clone();

        for circle in circles {
            let distance_between_centers = bounding_circle.center.distance(&circle.center);
            if bounding_circle.radius < distance_between_centers + circle.radius {
                // circle not contained in bounding circle, expand
                let diameter = Edge::new(bounding_circle.center, circle.center)
                    .extend_at_front(bounding_circle.radius)
                    .extend_at_back(circle.radius);

                let new_radius = diameter.diameter() / 2.0;
                let new_center = diameter.centroid();

                bounding_circle = Circle::new(new_center, new_radius);
            }
        }
        bounding_circle
    }
}

impl Transformable for Circle {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        let Circle { center, radius: _ } = self;
        center.transform(t);
        self
    }
}

impl TransformableFrom for Circle {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        let Circle { center, radius: _ } = self;
        center.transform_from(&reference.center, t);
        self
    }
}

impl CollidesWith<Circle> for Circle {
    fn collides_with(&self, other: &Circle) -> bool {
        let (cx1, cx2) = (self.center.0, other.center.0);
        let (cy1, cy2) = (self.center.1, other.center.1);
        let (r1, r2) = (self.radius, other.radius);

        let dx = cx1 - cx2;
        let dy = cy1 - cy2;
        let sq_d = dx * dx + dy * dy;

        sq_d <= (r1 + r2) * (r1 + r2)
    }
}

impl CollidesWith<Edge> for Circle {
    fn collides_with(&self, edge: &Edge) -> bool {
        edge.sq_distance(&self.center) <= self.radius.powi(2)
    }
}

impl CollidesWith<AARectangle> for Circle {
    #[inline(always)]
    fn collides_with(&self, rect: &AARectangle) -> bool {
        //Based on: https://yal.cc/rectangle-circle-intersection-test/

        let Point(c_x, c_y) = self.center;

        let nearest_x = fsize::max(rect.x_min, fsize::min(c_x, rect.x_max));
        let nearest_y = fsize::max(rect.y_min, fsize::min(c_y, rect.y_max));

        (nearest_x - c_x).powi(2) + (nearest_y - c_y).powi(2) <= self.radius.powi(2)
    }
}

impl CollidesWith<Point> for Circle {
    fn collides_with(&self, point: &Point) -> bool {
        point.sq_distance(&self.center) <= self.radius.powi(2)
    }
}

impl DistanceFrom<Point> for Circle {
    fn sq_distance(&self, other: &Point) -> fsize {
        self.distance(other).powi(2)
    }

    fn distance(&self, point: &Point) -> fsize {
        let Point(x, y) = point;
        let Point(cx, cy) = self.center;
        let sq_d = (x - cx).powi(2) + (y - cy).powi(2);
        return if sq_d < self.radius.powi(2) {
            0.0 //point is inside circle
        } else {
            //point is outside circle
            fsize::sqrt(sq_d) - self.radius
        };
    }

    fn distance_from_border(&self, point: &Point) -> (GeoPosition, fsize) {
        let Point(x, y) = point;
        let Point(cx, cy) = self.center;
        let d_center = fsize::sqrt((x - cx).powi(2) + (y - cy).powi(2));
        match d_center.partial_cmp(&self.radius).unwrap() {
            Ordering::Less | Ordering::Equal => (GeoPosition::Interior, self.radius - d_center),
            Ordering::Greater => (GeoPosition::Exterior, d_center - self.radius),
        }
    }

    fn sq_distance_from_border(&self, point: &Point) -> (GeoPosition, fsize) {
        let (pos, distance) = self.distance_from_border(point);
        (pos, distance.powi(2))
    }
}

impl Shape for Circle {
    fn centroid(&self) -> Point {
        self.center.clone()
    }

    fn area(&self) -> fsize {
        self.radius * self.radius * PI
    }

    fn bbox(&self) -> AARectangle {
        let (r, x, y) = (self.radius, self.center.0, self.center.1);
        AARectangle::new(x - r, y - r, x + r, y + r)
    }

    fn diameter(&self) -> fsize {
        self.radius * 2.0
    }
}
