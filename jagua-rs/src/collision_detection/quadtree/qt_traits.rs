use crate::geometry::geo_traits::CollidesWith;
#[cfg(doc)]
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::Rect;

/// Common trait for all geometric primitives that can be directly queried in the quadtree
/// for collisions with the edges of the registered hazards. These include: [Rect], [Edge] and [Circle].
pub trait QTQueryable: CollidesWith<Edge> + CollidesWith<Rect> {}

// Blanket implementation for any type that satisfies the trait bounds.
impl<T> QTQueryable for T where T: CollidesWith<Edge> + CollidesWith<Rect> {}
