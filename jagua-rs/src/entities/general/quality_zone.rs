use std::sync::Arc;

use crate::geometry::primitives::simple_polygon::SimplePolygon;

/// Maximum number of qualities that can be used
pub const N_QUALITIES: usize = 10;

/// Represents a zone of inferior quality in the `Bin`
#[derive(Clone, Debug)]
pub struct InferiorQualityZone {
    /// Higher quality is better
    pub quality: usize,
    /// The outer shapes of all zones of this quality
    pub zones: Vec<Arc<SimplePolygon>>,
}

impl InferiorQualityZone {
    pub fn new(quality: usize, shapes: Vec<SimplePolygon>) -> Self {
        assert!(
            quality < N_QUALITIES,
            "Quality must be in range of N_QUALITIES"
        );
        let zones = shapes.into_iter().map(Arc::new).collect();
        Self { quality, zones }
    }
}
