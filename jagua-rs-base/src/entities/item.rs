use std::sync::Arc;

use crate::collision_detection::hazards::filter::QZHazardFilter;
use crate::entities::original_shape::OriginalShape;
use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::geo_enums::RotationRange;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::SPolygon;

/// Item to be produced.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Contour of the item as defined in the input file
    pub shape_orig: Arc<OriginalShape>,
    /// Contour of the item to be used for collision detection
    pub shape_cd: Arc<SPolygon>,
    /// Possible rotations in which to place the item
    pub allowed_rotation: RotationRange,
    /// The minimum quality the item should be produced out of, if `None` the item requires full quality
    pub min_quality: Option<usize>,
    /// Filter for hazards that the item is unaffected by
    pub hazard_filter: Option<QZHazardFilter>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
}

impl Item {
    pub fn new(
        id: usize,
        original_shape: OriginalShape,
        allowed_rotation: RotationRange,
        base_quality: Option<usize>,
        surrogate_config: SPSurrogateConfig,
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
            min_quality: base_quality,
            hazard_filter,
            surrogate_config,
        }
    }

    pub fn area(&self) -> f32 {
        self.shape_orig.area()
    }
}
