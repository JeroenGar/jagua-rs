use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::primitives::{Circle, Edge};

/// TODO: document
#[derive(Clone, Debug)]
pub struct PSurrogate {
    pub poles: Vec<Circle>,
    pub piers: Vec<Edge>,
    pub convex_hull_indices: Vec<usize>,
    pub convex_hull_area: f32,
    pub config: SPSurrogateConfig,
}