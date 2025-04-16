use std::sync::Arc;

use crate::collision_detection::hazards::filter::QZHazardFilter;
use crate::entities::general::original_shape::OriginalShape;
use crate::fsize;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::primitives::SimplePolygon;
use crate::util::SPSurrogateConfig;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Contour of the item as defined in the input file
    pub original_shape: Arc<OriginalShape>,
    /// Contour of the item to be used internally
    pub shape: Arc<SimplePolygon>,
    /// Possible rotations in which to place the item
    pub allowed_rotation: AllowedRotation,
    /// The quality of the item, if `None` the item requires full quality
    pub base_quality: Option<usize>,
    /// Filter for hazards that the item is unaffected by
    pub hazard_filter: Option<QZHazardFilter>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
    /// Value of the item
    pub value: u64,
}

impl Item {
    pub fn new(
        id: usize,
        original_shape: OriginalShape,
        allowed_rotation: AllowedRotation,
        base_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
        value: u64,
    ) -> Item {
        let mut shape = original_shape.convert_to_internal();
        shape.generate_surrogate(surrogate_config);
        let original_shape = Arc::new(original_shape);
        let shape = Arc::new(shape);
        let hazard_filter = base_quality.map(QZHazardFilter);
        Item {
            id,
            shape,
            original_shape,
            allowed_rotation,
            base_quality,
            hazard_filter,
            surrogate_config,
            value,
        }
    }

    pub fn area(&self) -> fsize {
        self.original_shape.area()
    }
}
