use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::point::Point;
use crate::geometry::transformation::Transformation;

/// Trait for types that can detect collisions between itself and an object from type T.
pub trait CollidesWith<T> {
    fn collides_with(&self, other: &T) -> bool;
}

/// Trait for types that can detect almost-collisions between itself and an object from type T.
/// Useful in situations where fp arithmetic precision could be problematic.
/// Leans towards false positives rather than false negatives.
pub trait AlmostCollidesWith<T> {
    fn almost_collides_with(&self, other: &T) -> bool;
}

/// Trait for geometric primitives that can calculate distances to other primitives.
pub trait DistanceFrom<T> {
    fn sq_distance(&self, other: &T) -> f64;
    fn distance(&self, other: &T) -> f64;
    fn distance_from_border(&self, other: &T) -> (GeoPosition, f64);
    fn sq_distance_from_border(&self, other: &T) -> (GeoPosition, f64);
}

/// Trait for types that can be transformed by a Transformation.
pub trait Transformable: Clone {
    fn transform(&mut self, t: &Transformation) -> &mut Self;

    fn transform_clone(&self, t: &Transformation) -> Self {
        let mut clone = self.clone();
        clone.transform(t);
        clone
    }
}

/// Trait for types that can be transformed based on a reference object with a Transformation applied.
/// Used for repeated transformations on an identical reference shape without reallocating new memory each time.
pub trait TransformableFrom: Transformable {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self;
}

/// Trait for shared properties of geometric primitives.
pub trait Shape {

    /// Geometric center of the shape
    fn centroid(&self) -> Point;

    /// Area of the interior of the shape
    fn area(&self) -> f64;

    /// Bounding box of the shape
    fn bbox(&self) -> AARectangle;

    /// The distance between the two furthest points in the shape.
    fn diameter(&self) -> f64;
}
