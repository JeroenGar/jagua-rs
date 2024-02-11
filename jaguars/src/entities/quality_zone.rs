use std::sync::Arc;

use crate::geometry::primitives::simple_polygon::SimplePolygon;

/// Maximum number of qualities that can be used
pub const N_QUALITIES: usize = 10;

/// Represents a zone of certain quality in the `Bin`
#[derive(Clone, Debug)]
pub struct QualityZone {
    /// Higher quality is better
    pub quality: usize,
    /// The outer shapes of all zones of this quality
    pub zones: Vec<Arc<SimplePolygon>>,
}

impl QualityZone {
    pub fn new(quality: usize, shapes: Vec<SimplePolygon>) -> Self {
        assert!(quality < N_QUALITIES, "Quality must be less than N_QUALITIES");
        let zones = shapes.into_iter().map(|z| Arc::new(z)).collect();
        Self { quality, zones }
    }
}