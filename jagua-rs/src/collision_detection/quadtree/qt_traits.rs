use crate::geometry::geo_traits::{CollidesWith, Shape};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;

/// Trait for all geometric primitives that can be directly queried in the quadtree for collisions
pub trait QTQueryable: CollidesWith<AARectangle> + CollidesWith<Edge> + Shape {}

impl QTQueryable for AARectangle {}
impl QTQueryable for Edge {}
impl QTQueryable for Circle {}
