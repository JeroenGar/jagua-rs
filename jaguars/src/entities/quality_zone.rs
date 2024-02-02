use std::sync::Arc;

use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::N_QUALITIES;

#[derive(Clone, Debug)]
pub struct QualityZone {
    /// Higher quality is better
    quality: usize,
    shapes: Vec<Arc<SimplePolygon>>,
}

impl QualityZone {
    pub fn new(quality: usize, shapes: Vec<SimplePolygon>) -> Self {
        assert!(quality < N_QUALITIES, "Quality must be less than N_QUALITIES");
        let shapes = shapes.into_iter().map(|z| Arc::new(z)).collect();
        Self { quality, shapes }
    }
    pub fn quality(&self) -> usize {
        self.quality
    }

    pub fn shapes(&self) -> &Vec<Arc<SimplePolygon>> {
        &self.shapes
    }
}