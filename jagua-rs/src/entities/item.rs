use std::sync::Arc;

use crate::collision_detection::hazard_filter::QZHazardFilter;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::config::SPSurrogateConfig;

/// An `Item` to be placed in a `Bin`.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub shape: Arc<SimplePolygon>,
    pub allowed_rotation: AllowedRotation,
    pub base_quality: Option<usize>,
    pub value: u64,
    pub centering_transform: Transformation,
    pub hazard_filter: Option<QZHazardFilter>,
}

impl Item {
    pub fn new(
        id: usize,
        mut shape: SimplePolygon,
        value: u64,
        allowed_rotation: AllowedRotation,
        centering_transform: Transformation,
        base_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
    ) -> Item {
        shape.generate_surrogate(surrogate_config);
        let shape = Arc::new(shape);
        let hazard_filter = base_quality.map(|q| QZHazardFilter { cutoff_quality: q });
        Item {
            id,
            shape,
            allowed_rotation,
            base_quality,
            value,
            centering_transform,
            hazard_filter,
        }
    }

    pub fn clone_with_id(&self, id: usize) -> Item {
        Item { id, ..self.clone() }
    }
}
