use std::sync::Arc;

use crate::collision_detection::hazards::filter::QZHazardFilter;
use crate::entities::general::original_shape::OriginalShape;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::SPolygon;
use crate::util::SPSurrogateConfig;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Contour of the item as defined in the input file
    pub shape_orig: Arc<OriginalShape>,
    /// Contour of the item to be used for collision detection
    pub shape_cd: Arc<SPolygon>,
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
        let shape_orig = Arc::new(original_shape);
        let shape_int = {
            let mut shape_int = shape_orig.convert_to_internal();
            shape_int.generate_surrogate(surrogate_config);
            Arc::new(shape_int)
        };
        let hazard_filter = base_quality.map(QZHazardFilter);
        Item {
            id,
            shape_orig,
            shape_cd: shape_int,
            allowed_rotation,
            base_quality,
            hazard_filter,
            surrogate_config,
            value,
        }
    }

    pub fn area(&self) -> f32 {
        self.shape_orig.area()
    }
}
