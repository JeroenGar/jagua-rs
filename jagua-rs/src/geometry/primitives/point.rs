use std::hash::{Hash, Hasher};

use crate::fsize;
use crate::geometry::Transformation;
use crate::geometry::geo_traits::{CollidesWith, DistanceTo, Transformable, TransformableFrom};

/// Point(x, y)
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point(pub fsize, pub fsize);

impl Transformable for Point {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        let Point(x, y) = self;
        (*x, *y) = TRANSFORM_FORMULA(*x, *y, t);
        self
    }
}

impl TransformableFrom for Point {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        let Point(x, y) = self;
        (*x, *y) = TRANSFORM_FORMULA(reference.0, reference.1, t);
        self
    }
}

const TRANSFORM_FORMULA: fn(fsize, fsize, &Transformation) -> (fsize, fsize) =
    |x, y, t| -> (fsize, fsize) {
        let m = t.matrix();
        let t_x = m[0][0].into_inner() * x + m[0][1].into_inner() * y + m[0][2].into_inner() * 1.0;
        let t_y = m[1][0].into_inner() * x + m[1][1].into_inner() * y + m[1][2].into_inner() * 1.0;
        (t_x, t_y)
    };

impl Point {
    pub fn x(&self) -> fsize {
        self.0
    }

    pub fn y(&self) -> fsize {
        self.1
    }
}

impl DistanceTo<Point> for Point {
    #[inline(always)]
    fn distance_to(&self, other: &Point) -> fsize {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }

    #[inline(always)]
    fn sq_distance_to(&self, other: &Point) -> fsize {
        (self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = self.0.to_bits();
        let y = self.1.to_bits();
        x.hash(state);
        y.hash(state);
    }
}

impl From<Point> for (fsize, fsize) {
    fn from(p: Point) -> Self {
        (p.0, p.1)
    }
}

impl From<(fsize, fsize)> for Point {
    fn from((x, y): (fsize, fsize)) -> Self {
        Point(x, y)
    }
}

impl<T> CollidesWith<T> for Point
where
    T: CollidesWith<Point>,
{
    fn collides_with(&self, other: &T) -> bool {
        other.collides_with(self)
    }
}
