use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::transformation::Transformation;

pub trait CollidesWith<T> {
    fn collides_with(&self, other: &T) -> bool;
}

pub trait AlmostCollidesWith<T> {
    //for use is situations where limited FP precision is problematic.
    //Leans towards false positives rather than false negatives
    fn almost_collides_with(&self, other: &T) -> bool;
}

pub trait DistanceFrom<T> {
    fn sq_distance(&self, other: &T) -> f64;
    fn distance(&self, other: &T) -> f64;
    fn distance_from_border(&self, other: &T) -> (GeoPosition, f64);
    fn sq_distance_from_border(&self, other: &T) -> (GeoPosition, f64);
}

pub trait Transformable: Clone {
    fn transform(&mut self, t: &Transformation) -> &mut Self;

    fn transform_clone(&self, t: &Transformation) -> Self {
        let mut clone = self.clone();
        clone.transform(t);
        clone
    }
}

pub trait TransformableFrom: Transformable {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self;
}

pub trait Shape {
    fn centroid(&self) -> Point;

    fn area(&self) -> f64;

    fn bbox(&self) -> AARectangle;

    fn diameter(&self) -> f64;
}
