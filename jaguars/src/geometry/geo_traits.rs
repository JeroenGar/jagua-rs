use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;
use crate::geometry::transformation::Transformation;

/// Trait for types that can detect collisions between itself and on object from type T
pub trait CollidesWith<T> {
    fn collides_with(&self, other: &T) -> bool;
}

/// Trait for types that can detect almost-collisions between itself and on object from type T
/// Useful in situations where fp arithmetic precision could be problematic
/// Leans towards false positives rather than false negatives
pub trait AlmostCollidesWith<T> {
    fn almost_collides_with(&self, other: &T) -> bool;
}

/// Trait for geometric primitives that can calculate distances to other objects
pub trait DistanceFrom<T> {
    fn sq_distance(&self, other: &T) -> f64;
    fn distance(&self, other: &T) -> f64;
    fn distance_from_border(&self, other: &T) -> (GeoPosition, f64);
    fn sq_distance_from_border(&self, other: &T) -> (GeoPosition, f64);
}

/// Trait for types that can be transformed by a Transformation
pub trait Transformable: Clone {
    fn transform(&mut self, t: &Transformation) -> &mut Self;

    fn transform_clone(&self, t: &Transformation) -> Self {
        let mut clone = self.clone();
        clone.transform(t);
        clone
    }
}

/// Trait for types that can be transformed based on a reference object with a Transformation applied
pub trait TransformableFrom: Transformable {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self;
}

/// Trait for shared properties of geometric primitives
pub trait Shape {
    fn centroid(&self) -> Point;

    fn area(&self) -> f64;

    fn bbox(&self) -> AARectangle;

    fn diameter(&self) -> f64;
}
