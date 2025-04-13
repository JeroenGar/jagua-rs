use crate::fsize;
use crate::geometry::Transformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::Point;

/// Trait for types that can detect collisions between `Self` and `T`.
pub trait CollidesWith<T> {
    fn collides_with(&self, other: &T) -> bool;
}

///  Trait for types that can detect 'almost-collisions' between `Self` and `T`.
///
/// Due to floating point arithmetic precision, two objects that are very close to each other may have unexpected behavior with
/// the [CollidesWith] trait. This trait errors on the side of false positives, so that if two objects are very close to each other,
/// they will be considered as colliding.
pub trait AlmostCollidesWith<T> {
    fn almost_collides_with(&self, other: &T) -> bool;
}

/// Trait for types that can compute the minimum distance between `Self` and `T`.
pub trait DistanceTo<T> {
    /// Minimum distance between two primitives. Will be 0 in case of a collision.
    fn distance(&self, other: &T) -> fsize;

    /// Squared version of [DistanceTo::distance]
    fn sq_distance(&self, other: &T) -> fsize;
}

/// Trait for types that can compute the minimum distance to separate `Self` from `T`.
pub trait SeparationDistance<T>: DistanceTo<T> {
    /// In case of a collision between `self` and `other`, returns [GeoPosition::Interior] and the minimum distance to separate the two primitives.
    /// Otherwise, returns [GeoPosition::Exterior] and the minimum distance between the two primitives. (similar to [DistanceTo::distance])
    fn separation_distance(&self, other: &T) -> (GeoPosition, fsize);

    /// Squared version of [SeparationDistance::separation_distance]
    fn sq_separation_distance(&self, other: &T) -> (GeoPosition, fsize);
}

/// Trait for types that can be modified by a [`Transformation`].
pub trait Transformable: Clone {
    /// Applies a transformation to `self`.
    fn transform(&mut self, t: &Transformation) -> &mut Self;

    /// Applies a transformation to a clone.
    fn transform_clone(&self, t: &Transformation) -> Self {
        let mut clone = self.clone();
        clone.transform(t);
        clone
    }
}

/// Trait for types that can be modified based on a reference object with a [`Transformation`] applied.
///
/// Useful when repeatedly transforming a single shape without having to reallocate new memory each time.
pub trait TransformableFrom: Transformable {
    /// Applies a transformation on the reference object and stores the result in `self`.
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self;
}

/// Trait for shared properties of geometric primitives.
pub trait Shape {
    /// Geometric center of the shape
    fn centroid(&self) -> Point;

    /// Area of the interior of the shape
    fn area(&self) -> fsize;

    /// Bounding box of the shape
    fn bbox(&self) -> AARectangle;

    /// The distance between the two furthest points in the shape.
    fn diameter(&self) -> fsize;
}
