use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::Edge;
#[cfg(doc)]
use crate::geometry::primitives::Circle;

/// Common trait for all geometric primitives that can be directly queried in the quadtree
/// for collisions with the edges of the registered hazards. These include: [AARectangle], [Edge] and [Circle].
pub trait QTQueryable: Shape + CollidesWith<Edge> + CollidesWith<AARectangle> {}

// Blanket implementation for any type that satisfies the trait bounds.
impl<T> QTQueryable for T
where
    T: Shape + CollidesWith<Edge> + CollidesWith<AARectangle>,
{}