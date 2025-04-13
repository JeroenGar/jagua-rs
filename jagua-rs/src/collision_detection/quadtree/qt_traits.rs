use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::AARectangle;
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Edge;

/// Common trait for all geometric primitives that can be directly queried in the quadtree
/// for collisions with the edges of the registered hazards.
/// These include: [AARectangle], [Edge] and [Circle].
pub trait QTQueryable: Shape + CollidesWith<Edge> + CollidesWith<AARectangle> {}

impl QTQueryable for AARectangle {}
impl QTQueryable for Edge {}
impl QTQueryable for Circle {}
